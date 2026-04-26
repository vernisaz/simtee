use simcli::{CliNoMut, OptTyp, OptVal};
use std::{
    fs::File,
    io::Write,
    io::{self, Read},
    time::{SystemTime, UNIX_EPOCH},
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
        .opt("-version", OptTyp::None)?
        .opt("h", OptTyp::None)?
        .description("Help for the utility");
    if cli.get_opt("v").is_some() {
        println!(
            "Simple Tee version {}, copyright © {} D. Rogatkin",
            env!("VERSION"),
            year_now()
        );
        return Ok(());
    } else if cli.get_opt("h").is_some() {
        println!(
            "Simple Tee\nUsage: simtee [options...] [<file>...]\nWhere options are:{}",
            cli.get_description().ok_or("no help specified")?
        );
        return Ok(());
    }
    if let Some(invalid_opts) = cli.get_errors() {
        return Err(
            format!("Some unrecognized option(s) {invalid_opts:?} ... was specified").into(),
        );
    }
    const SIZE: usize = 1024 * 512;
    let mut buffer = [0u8; SIZE]; // Fixed-size array initialized with zeros
    let overwrite = cli.get_opt("w").is_some();
    let mut out: Box<dyn Write> = if let Some(OptVal::Str(name)) = cli.get_opt("o")
        && cli.get_opt("r").is_none()
    {
        Box::new(
            File::options()
                .truncate(overwrite)
                .append(!overwrite)
                .write(true)
                .create(!overwrite)
                .open(name)?,
        )
    } else {
        Box::new(io::stdout())
    };
    // create a vec of files for reverse (-r) operation
    let mut out_files = Vec::with_capacity(cli.args().len());
    if cli.get_opt("r").is_some() && cli.get_opt("o").is_none() {
        let append = cli.get_opt("a").is_some();
        if overwrite && append {
            return Err("Overwrite and append options can't be applied together".into());
        }
        for f in cli.args() {
            match File::options()
                .truncate(overwrite)
                .append(append)
                .write(!append || overwrite)
                .create_new(!append)
                .open(&f)
            {
                Ok(f) => out_files.push(f),
                Err(err) => eprintln!("Can't open {f} for writing: {err}"),
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
                eprintln!("Error reading from stdin: {e}");
                break; //std::process::exit(1);
            }
        }
    }
    if cli.get_opt("r").is_none() {
        for f in cli.args() {
            let mut file = match File::options().read(true).open(&f) {
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
                        eprintln!("Error reading from {f}: {e}");
                        break; //std::process::exit(1);
                    }
                }
            }
        }
    }
    Ok(())
}

#[inline]
pub fn year_now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
        / 31556952
        + 1970
}
