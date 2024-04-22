use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, Write};

use anyhow::{anyhow, Result};
use clap::Parser;

#[derive(Debug, Parser)]
#[command(version, about, author)]
struct Args {
    #[arg(value_name = "IN_FILE", default_value = "-")]
    in_file: String,
    #[arg(value_name = "OUT_FILE")]
    out_file: Option<String>,
    #[arg(short, long)]
    count: bool,
}

fn main() {
    if let Err(e) = run(Args::parse()) {
        eprintln!("{e}");
        std::process::exit(1);
    }
}

fn run(args: Args) -> Result<()> {
    let mut file = open(&args.in_file).map_err(|e| anyhow!("{}: {e}", args.in_file))?;
    let mut out_file: Box<dyn Write> = match &args.out_file {
        Some(out_file) => Box::new(File::create(out_file)?),
        None => Box::new(io::stdout()),
    };

    let mut line = String::new();
    let mut current_line: Option<String> = None;
    let mut current_count = 1;

    loop {
        line.clear();
        let bytes = file.read_line(&mut line)?;
        if bytes == 0 {
            break;
        }

        match &mut current_line {
            None => {
                let current_line = current_line.insert(String::new());
                std::mem::swap(current_line, &mut line);
            }
            Some(current_line) if current_line.trim_end() == line.trim_end() => {
                current_count += 1;
            }
            Some(current_line) => {
                if args.count {
                    write!(&mut out_file, "{:4} {}", current_count, current_line)?;
                } else {
                    write!(&mut out_file, "{}", current_line)?;
                }
                current_count = 1;
                std::mem::swap(current_line, &mut line);
            }
        }
    }

    if let Some(current_line) = current_line {
        if args.count {
            write!(&mut out_file, "{:4} {}", current_count, current_line)?;
        } else {
            write!(&mut out_file, "{}", current_line)?;
        }
    }

    Ok(())
}

fn open(filename: &str) -> Result<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}
