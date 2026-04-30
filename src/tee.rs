#[cfg(not(target_os = "windows"))]
use simcli::{CliNoMut, OptTyp, OptVal};
#[cfg(target_os = "windows")]
use simcli::{CliNoMut, OptTyp, OptVal, WildCardExpansion};
use simcolor::Colorized;
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
    #[cfg(target_os = "windows")]
    cli.process_wildcard(WildCardExpansion::All);
    if cli.get_opt("v").is_some() {
        println!(
            "{} version {}, Copyright © {} D. Rogatkin",
            env!("NAME").blue().bright().bold(),
            env!("VERSION").green(),
            year_now().bright().magenta()
        );
        return Ok(());
    } else if cli.get_opt("h").is_some() {
        println!(
            "Usage: simtee [options...] [<file>...]\nWhere options are:{}",
            cli.get_description()
                .ok_or("no help specified")?
                .bright()
                .blue()
        );
        return Ok(());
    }
    if let Some(invalid_opts) = cli.get_errors() {
        return Err(Box::new(
            format!(
                "Some unrecognized option(s) '{}' ... was specified",
                invalid_opts.join(", ")
            )
            .red(),
        ));
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
            return Err(Box::new(
                "Overwrite and append options can't be applied together".red(),
            ));
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
                Err(err) => eprintln!(
                    "Can't open {} for writing: {}",
                    f.blue(),
                    err.to_string().red()
                ),
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
                eprintln!("Error reading from stdin: {}", e.to_string().red());
                break; //std::process::exit(1);
            }
        }
    }
    if cli.get_opt("r").is_none() {
        for f in cli.args() {
            let mut file = match File::options().read(true).open(&f) {
                Ok(file) => file,
                Err(e) => {
                    eprintln!("Error: {} reading from {f}", e.to_string().red());
                    continue;
                }
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
                        eprintln!("Error reading from {}: {}", f.blue(), e.to_string().red());
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
