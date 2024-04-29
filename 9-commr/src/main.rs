use crate::Column::{Col1, Col2, Col3};
use anyhow::{anyhow, bail, Result};
use clap::Parser;
use io::BufRead;
use std::cmp::Ordering;
use std::io::BufReader;
use std::{fs, io};

#[derive(Debug, Parser)]
pub struct Args {
    #[arg(value_name = "FILE1")]
    file1: String,
    #[arg(value_name = "FILE2")]
    file2: String,
    #[arg(short = '1', action = clap::ArgAction::SetFalse)]
    show_col1: bool,
    #[arg(short = '2', action = clap::ArgAction::SetFalse)]
    show_col2: bool,
    #[arg(short = '3', action = clap::ArgAction::SetFalse)]
    show_col3: bool,
    #[arg(short)]
    insensitive: bool,
    #[arg(short, long("output-delimiter"), default_value = "\t")]
    delimiter: String,
}

fn main() {
    let args = Args::parse();
    if let Err(e) = run(args) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}

fn run(args: Args) -> Result<()> {
    if args.file1 == "-" && args.file2 == "-" {
        bail!(r#"Both input files cannot be STDIN ("-")"#);
    }

    let print = |col: Column| {
        let mut columns = vec![];
        match col {
            Col1(val) => {
                if args.show_col1 {
                    columns.push(val);
                }
            }
            Col2(val) => {
                if args.show_col2 {
                    if args.show_col1 {
                        columns.push("");
                    }
                    columns.push(val);
                }
            }
            Col3(val) => {
                if args.show_col3 {
                    if args.show_col1 {
                        columns.push("");
                    }
                    if args.show_col2 {
                        columns.push("");
                    }
                    columns.push(val);
                }
            }
        };

        if !columns.is_empty() {
            println!("{}", columns.join(&args.delimiter));
        }
    };

    let mut file1 = open(&args.file1)?.lines();
    let mut file2 = open(&args.file2)?.lines();

    let mut line1_opt = file1.next().transpose()?;
    let mut line2_opt = file2.next().transpose()?;

    loop {
        match (&mut line1_opt, &mut line2_opt) {
            (Some(line1), Some(line2)) => {
                let cmp = if args.insensitive {
                    line1.to_lowercase().cmp(&line2.to_lowercase())
                } else {
                    line1.cmp(&line2)
                };
                match cmp {
                    Ordering::Less => {
                        print(Col1(&line1));
                        line1_opt = file1.next().transpose()?;
                    }
                    Ordering::Equal => {
                        print(Col3(&line1));
                        line1_opt = file1.next().transpose()?;
                        line2_opt = file2.next().transpose()?;
                    }
                    Ordering::Greater => {
                        print(Col2(&line2));
                        line2_opt = file2.next().transpose()?;
                    }
                }
            }
            (Some(line1), None) => {
                print(Col1(line1));
                line1_opt = file1.next().transpose()?;
            }
            (None, Some(line2)) => {
                print(Col2(line2));
                line2_opt = file2.next().transpose()?;
            }
            _ => break,
        }
    }

    Ok(())
}

enum Column<'a> {
    Col1(&'a str),
    Col2(&'a str),
    Col3(&'a str),
}

fn open(file: &str) -> Result<Box<dyn BufRead>> {
    if file == "-" {
        Ok(Box::new(BufReader::new(io::stdin())))
    } else {
        Ok(Box::new(BufReader::new(
            fs::File::open(file).map_err(|e| anyhow!("{}: {}", file, e))?,
        )))
    }
}
