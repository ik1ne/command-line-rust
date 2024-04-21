use anyhow::Result;
use clap::Parser;
use std::fs::File;
use std::io::{stdin, BufRead, BufReader};
use std::iter::Sum;
use std::ops::Add;

#[derive(Debug, Parser)]
#[command(version, about, author)]
struct Args {
    #[arg(value_name = "FILE", default_value = "-")]
    files: Vec<String>,
    #[arg(short, long)]
    lines: bool,
    #[arg(short, long)]
    words: bool,
    #[arg(short = 'c', long, conflicts_with = "chars")]
    bytes: bool,
    #[arg(short = 'm', long)]
    chars: bool,
}

impl Args {
    fn turn_on_all_flags_if_none(&mut self) {
        if !self.lines && !self.words && !self.bytes && !self.chars {
            self.lines = true;
            self.words = true;
            self.bytes = true;
        }
    }
}

fn main() {
    if let Err(e) = run(Args::parse()) {
        eprintln!("{e}");
        std::process::exit(1);
    }
}

fn run(mut args: Args) -> Result<()> {
    args.turn_on_all_flags_if_none();

    let results = args
        .files
        .iter()
        .map(|filename| process_single_file(filename, &args))
        .collect::<Result<Vec<_>>>()?;

    if results.len() > 1 {
        let total: Counts = results.into_iter().sum();
        total.report(&args, "total");
    }

    Ok(())
}

#[derive(Debug, Default, Clone, Copy, Eq, PartialEq)]
struct Counts {
    lines: usize,
    words: usize,
    bytes: usize,
    chars: usize,
}

impl Counts {
    fn report(&self, args: &Args, filename: &str) {
        if args.lines {
            print!("{:8}", self.lines);
        }
        if args.words {
            print!("{:8}", self.words);
        }
        if args.bytes {
            print!("{:8}", self.bytes);
        }
        if args.chars {
            print!("{:8}", self.chars);
        }

        if filename != "-" {
            println!(" {filename}");
        } else {
            println!();
        }
    }
}

impl Sum for Counts {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Counts::default(), |acc, x| acc + x)
    }
}

impl Add for Counts {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Counts {
            lines: self.lines + rhs.lines,
            words: self.words + rhs.words,
            bytes: self.bytes + rhs.bytes,
            chars: self.chars + rhs.chars,
        }
    }
}

fn process_single_file(filename: &str, args: &Args) -> Result<Counts> {
    let reader = match open(filename) {
        Ok(reader) => reader,
        Err(e) => {
            eprintln!("{filename}: {e}");
            return Ok(Counts::default());
        }
    };

    let counts = count_bufread(reader)?;

    counts.report(args, filename);

    Ok(counts)
}

fn count_bufread(mut reader: impl BufRead) -> Result<Counts> {
    let mut buf = vec![];
    let mut counts = Counts::default();

    loop {
        buf.clear();
        let bytes_read = reader.read_until(b'\n', &mut buf)?;
        if bytes_read == 0 {
            break;
        }

        let line = String::from_utf8_lossy(&buf);
        counts.lines += 1;
        counts.words += line.split_whitespace().count();
        counts.bytes += bytes_read;
        counts.chars += line.chars().count();
    }

    Ok(counts)
}

fn open(filename: &str) -> Result<Box<dyn BufRead>> {
    if filename == "-" {
        Ok(Box::new(BufReader::new(stdin())))
    } else {
        let file = File::open(filename)?;
        Ok(Box::new(BufReader::new(file)))
    }
}

#[cfg(test)]
mod tests;
