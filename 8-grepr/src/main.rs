use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};

use anyhow::{Context, Result};
use clap::Parser;
use regex::RegexBuilder;

use crate::files::find_files;
use crate::find::find_lines;

mod files;
mod find;

#[derive(Debug, Parser)]
struct Args {
    #[arg(required = true)]
    pattern: String,
    #[arg(default_value = "-", value_name = "FILE")]
    files: Vec<String>,
    #[arg(short, long)]
    insensitive: bool,
    #[arg(short, long)]
    recursive: bool,
    #[arg(short, long)]
    count: bool,
    #[arg(short = 'v', long = "invert-match")]
    invert: bool,
}

fn main() {
    let args = Args::parse();
    if let Err(e) = run(args) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}

fn run(args: Args) -> Result<()> {
    let re = RegexBuilder::new(&args.pattern)
        .case_insensitive(args.insensitive)
        .build()
        .with_context(|| format!(r#"Invalid pattern "{}""#, args.pattern))?;

    let paths = find_files(&args.files, args.recursive);
    let file_count = paths.len();

    for path in paths {
        let (file, path) = match path {
            Ok(path) => (open(&path)?, path),
            Err(e) => {
                eprintln!("{e}");
                continue;
            }
        };

        let lines = find_lines(file, &re, args.invert)?;

        if args.count {
            if file_count > 1 {
                println!("{}:{}", path, lines.len());
            } else {
                println!("{}", lines.len());
            }
        } else {
            for line in lines {
                if file_count > 1 {
                    print!("{}:{}", path, line);
                } else {
                    print!("{}", line);
                }
            }
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
