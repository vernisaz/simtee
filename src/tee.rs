use simcli::{CliNoMut, OptTyp, OptVal};
use std::{
    fs::OpenOptions,
    io::Write,
    io::{self, Read},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = CliNoMut::new();
    cli.opt("o", OptTyp::Str)?
        .description("Out file name")
        .opt("r", OptTyp::None)?
        .description("Reverse operation")
        .opt("a", OptTyp::None)?
        .description("Append result")
        .opt("w", OptTyp::None)?
        .description("Overwrite result")
        .opt("v", OptTyp::None)?
        .description("Version number, all other operations ignored")
        .opt("h", OptTyp::None)?;
    if cli.get_opt("v").is_some() {
        println!("Simple Tee version {}", env!("VERSION"));
        return Ok(());
    } else if cli.get_opt("h").is_some() {
        println!(
            "Simple Tee, usage :  simtee [options...] [<file>...]\nWhere options:{}",
            cli.get_description().ok_or("no help specified")?
        );
        return Ok(());
    }
    const SIZE: usize = 1024 * 512;
    let mut buffer = [0u8; SIZE]; // Fixed-size array initialized with zeros
    let mut out: Box<dyn Write> = if let Some(OptVal::Str(name)) = cli.get_opt("o") {
        let overwrite = cli.get_opt("w").is_some();
        Box::new(
            OpenOptions::new()
                .truncate(overwrite)
                .write(true)
                .create(!overwrite)
                .open(name)?,
        )
    } else {
        Box::new(io::stdout())
    };
    // create a vec of files for -r operations
    let mut out_files = Vec::with_capacity(cli.args().len());
    if cli.get_opt("r").is_some() {
        let append = cli.get_opt("a").is_some();
        for f in cli.args() {
            if let Ok(f) = OpenOptions::new()
                .append(append)
                .write(!append)
                .create(!append)
                .open(f)
            {
                out_files.push(f)
            }
        }
    }
    loop {
        match io::stdin().read(&mut buffer) {
            Ok(bytes_read) => {
                if bytes_read == 0 {
                    break;
                };
                out.write_all(&buffer[..bytes_read])?;
                for mut w in &out_files {
                    w.write_all(&buffer[..bytes_read])?
                }
            }
            Err(e) => {
                eprintln!("Error reading from stdin: {}", e);
                break; //std::process::exit(1);
            }
        }
    }
    if cli.get_opt("r").is_none() {
        for f in cli.args() {
            let mut file = match OpenOptions::new().read(true).open(&f) {
                Ok(file) => file,
                _ => continue,
            };
            loop {
                match file.read(&mut buffer) {
                    Ok(bytes_read) => {
                        if bytes_read == 0 {
                            break;
                        };
                        out.write_all(&buffer[..bytes_read])?;
                    }
                    Err(e) => {
                        eprintln!("Error reading from {}: {}", f, e);
                        break; //std::process::exit(1);
                    }
                }
            }
        }
    }
    Ok(())
}
