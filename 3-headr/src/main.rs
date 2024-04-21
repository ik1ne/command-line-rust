use std::io;
use std::io::{BufRead, Read, Write};

use anyhow::Result;
use clap::Parser;

#[derive(Debug, Parser)]
struct Args {
    #[arg(default_value = "-", value_name = "FILE")]
    files: Vec<String>,
    #[arg(
        short = 'n',
        long,
        value_name = "LINES",
        conflicts_with = "bytes",
        default_value = "10",
        value_parser=clap::value_parser!(u64).range(1..),
    )]
    lines: u64,
    #[arg(
        short = 'c',
        long,
        value_name = "BYTES",
        value_parser=clap::value_parser!(u64).range(1..),
    )]
    bytes: Option<u64>,
}

fn main() {
    if let Err(e) = run(Args::parse()) {
        eprintln!("{e}");
        std::process::exit(1);
    }
}

fn run(args: Args) -> Result<()> {
    let num_files = args.files.len();

    for (i, filename) in args.files.into_iter().enumerate() {
        if num_files > 1 {
            if i > 0 {
                println!("\n==> {filename} <==")
            } else {
                println!("==> {filename} <==");
            }
        }

        let mut reader = match open(&filename) {
            Ok(file) => file,
            Err(e) => {
                eprintln!("{filename}: {e}");
                continue;
            }
        };

        if let Some(bytes) = args.bytes {
            let buf = reader
                .bytes()
                .take(bytes as usize)
                .collect::<Result<Vec<u8>, io::Error>>()?;
            io::stdout().lock().write_all(&buf)?;
        } else {
            let mut buffer = vec![];
            for _ in 0..(args.lines as usize) {
                buffer.clear();
                let bytes_read = reader.read_until(b'\n', &mut buffer)?;
                if bytes_read == 0 {
                    break;
                }

                io::stdout().lock().write_all(&buffer[..bytes_read])?;
            }
        }
    }

    Ok(())
}

fn open(filename: &str) -> Result<Box<dyn BufRead>> {
    if filename == "-" {
        Ok(Box::new(io::BufReader::new(io::stdin())))
    } else {
        Ok(Box::new(io::BufReader::new(std::fs::File::open(filename)?)))
    }
}
