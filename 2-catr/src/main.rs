use std::fs;
use std::io::{stdin, BufRead, BufReader};

use anyhow::Result;
use clap::{command, Parser};

#[derive(Debug, Parser)]
#[command(version, about, author)]
struct Args {
    #[arg(value_name = "FILE", default_value = "-")]
    files: Vec<String>,
    #[arg(short = 'n', long = "number", conflicts_with = "number_nonblank_lines")]
    number_lines: bool,
    #[arg(short = 'b', long = "number-nonblank")]
    number_nonblank_lines: bool,
}

fn main() {
    let args = Args::parse();
    if let Err(e) = run(args) {
        eprintln!("{e}");
        std::process::exit(1);
    }
}

fn run(args: Args) -> Result<()> {
    for filename in args.files {
        let mut line_num = 1;

        let reader = match open(&filename) {
            Ok(reader) => reader,
            Err(e) => {
                eprintln!("{filename}: {e}");
                return Ok(());
            }
        };

        for line in reader.lines() {
            let line = line?;
            if args.number_lines {
                println!("{:6}\t{}", line_num, line);
                line_num += 1;
            } else if args.number_nonblank_lines {
                if line.is_empty() {
                    println!();
                } else {
                    println!("{:6}\t{}", line_num, line);
                    line_num += 1;
                }
            } else {
                println!("{}", line);
            }
        }
    }

    Ok(())
}

fn open(filename: &str) -> Result<Box<dyn BufRead>> {
    if filename == "-" {
        Ok(Box::new(BufReader::new(stdin())))
    } else {
        Ok(Box::new(BufReader::new(fs::File::open(filename)?)))
    }
}
