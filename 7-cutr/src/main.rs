use crate::extract::{extract_bytes, extract_chars, extract_fields};
use anyhow::{bail, Result};
use clap::Parser;
use csv::{ReaderBuilder, WriterBuilder};
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};

use crate::position_list::PositionList;

mod extract;
mod position_list;

#[derive(Debug, Parser)]
#[clap(author, about, version)]
struct Args {
    #[arg(default_value = "-")]
    files: Vec<String>,
    #[arg(short, long, default_value = "\t")]
    delimiter: String,
    #[command(flatten)]
    extract: ArgsExtract,
}

#[derive(Debug, clap::Args)]
#[group(required = true, multiple = false)]
struct ArgsExtract {
    /// Selected fields
    #[arg(short, long, value_name = "FIELDS")]
    fields: Option<String>,
    /// Selected bytes
    #[arg(short, long, value_name = "BYTES")]
    bytes: Option<String>,
    /// Selected chars
    #[arg(short, long, value_name = "CHARS")]
    chars: Option<String>,
}

#[derive(Debug)]
pub enum Extract {
    Fields(PositionList),
    Bytes(PositionList),
    Chars(PositionList),
}

fn main() {
    let args = Args::parse();
    if let Err(e) = run(args) {
        eprintln!("{e}");
        std::process::exit(1);
    }
}

fn run(args: Args) -> Result<()> {
    let delimiter = parse_delimiter(&args.delimiter)?;

    let extract = match args.extract {
        ArgsExtract {
            fields: Some(fields),
            ..
        } => Extract::Fields(position_list::parse_pos(fields)?),
        ArgsExtract {
            bytes: Some(bytes), ..
        } => Extract::Bytes(position_list::parse_pos(bytes)?),
        ArgsExtract {
            chars: Some(chars), ..
        } => Extract::Chars(position_list::parse_pos(chars)?),
        _ => unreachable!("Must have --fields, --bytes, or --chars"),
    };

    for filename in &args.files {
        match open(filename) {
            Ok(file) => match &extract {
                Extract::Fields(fields) => {
                    let mut reader = ReaderBuilder::new()
                        .has_headers(false)
                        .delimiter(delimiter)
                        .from_reader(file);

                    let mut writer = WriterBuilder::new()
                        .delimiter(delimiter)
                        .from_writer(io::stdout());

                    for result in reader.records() {
                        let record = result?;
                        let fields = extract_fields(&record, fields);
                        writer.write_record(&fields)?;
                    }
                }
                Extract::Bytes(bytes) => {
                    for line in file.lines() {
                        let line = line?;
                        let extracted = extract_bytes(&line, bytes);
                        println!("{}", extracted);
                    }
                }
                Extract::Chars(chars) => {
                    for line in file.lines() {
                        let line = line?;
                        let extracted = extract_chars(&line, chars);
                        println!("{}", extracted);
                    }
                }
            },
            Err(e) => {
                eprintln!("{filename}: {e}");
                continue;
            }
        }
    }

    Ok(())
}

fn parse_delimiter(delim: &str) -> Result<u8> {
    let bytes = delim.as_bytes();

    if bytes.len() == 1 {
        Ok(bytes[0])
    } else {
        bail!("--delim \"{}\" must be a single byte", delim);
    }
}

fn open(filename: &str) -> Result<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}
