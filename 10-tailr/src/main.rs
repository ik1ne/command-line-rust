use crate::lines_bytes::LineCounter;
use crate::take_value::TakeValue;
use anyhow::{anyhow, Result};
use clap::Parser;
use std::fs::File;
use std::io::{BufRead, BufReader, Read, Seek};
use std::str::FromStr;

mod lines_bytes;
mod take_value;

#[derive(Debug, Parser)]
struct Args {
    #[arg(required = true)]
    files: Vec<String>,
    #[arg(short = 'n', long, default_value = "10")]
    lines: String,
    #[arg(short = 'c', long, conflicts_with = "lines")]
    bytes: Option<String>,
    #[arg(short, long)]
    quiet: bool,
}

fn main() {
    let args = Args::parse();
    if let Err(e) = run(args) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

fn run(args: Args) -> Result<()> {
    let lines =
        TakeValue::from_str(&args.lines).map_err(|e| anyhow!("illegal line count -- {e}"))?;

    let bytes = args
        .bytes
        .map(|b| TakeValue::from_str(&b))
        .transpose()
        .map_err(|e| anyhow!("illegal byte count -- {e}"))?;

    for (i, filename) in args.files.iter().enumerate() {
        let (file, counter) = match File::open(filename) {
            Ok(file) => (file, LineCounter::new(filename)?),
            Err(e) => {
                eprintln!("{filename}: {e}");
                continue;
            }
        };

        if !args.quiet && args.files.len() > 1 {
            println!("==> {} <==", filename);
        }

        match bytes {
            Some(b) => print_bytes(file, b, counter.bytes)?,
            None => print_lines(BufReader::new(file), lines, counter.lines)?,
        }
        if !args.quiet && i < args.files.len() - 1 {
            println!();
        }
    }
    Ok(())
}

fn print_lines(mut file: impl BufRead, lines: TakeValue, total_lines: usize) -> Result<()> {
    let start = get_start_index(lines, total_lines);

    let mut bytes = vec![];
    let mut current = 0;
    loop {
        let bytes_read = file.read_until(b'\n', &mut bytes)?;
        if bytes_read == 0 {
            break;
        }
        if current >= start {
            print!("{}", String::from_utf8_lossy(&bytes));
        }
        current += 1;
        bytes.clear();
    }

    Ok(())
}

fn print_bytes(mut file: impl Read + Seek, num_bytes: TakeValue, total_bytes: usize) -> Result<()> {
    let start = get_start_index(num_bytes, total_bytes);

    if start < total_bytes {
        file.seek(std::io::SeekFrom::Start(start as u64))?;
    } else {
        return Ok(());
    }

    let mut buffer = vec![0; total_bytes - start];
    file.read_exact(&mut buffer)?;

    print!("{}", String::from_utf8_lossy(&buffer));

    Ok(())
}

fn get_start_index(take_val: TakeValue, total: usize) -> usize {
    match take_val {
        TakeValue::FromEnd(n) => {
            if n >= total {
                0
            } else {
                total - n
            }
        }
        TakeValue::FromStart(n) => {
            if n > total {
                total
            } else {
                n.max(1) - 1
            }
        }
    }
}
