use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};

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
            Some(current_line) if current_line == &line => {
                current_count += 1;
            }
            Some(current_line) => {
                if args.count {
                    print!("{:4} {}", current_count, current_line);
                } else {
                    print!("{}", current_line);
                }
                current_count = 1;
                std::mem::swap(current_line, &mut line);
            }
        }
    }

    if let Some(current_line) = current_line {
        if args.count {
            print!("{:4} {}", current_count, current_line);
        } else {
            print!("{}", current_line);
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
