use std::fs;

use anyhow::Result;
use clap::builder::PossibleValue;
use clap::{ArgAction, Parser, ValueEnum};
use regex::Regex;
use walkdir::WalkDir;

#[derive(Debug, Parser)]
struct Args {
    #[arg(default_value = ".", help = "Search paths")]
    path: Vec<String>,
    #[arg(
        short,
        long = "name",
        value_parser = Regex::new,
        action = ArgAction::Append,
        num_args = 0..
    )]
    names: Vec<Regex>,
    #[arg(
        short = 't',
        long = "type",
        value_enum,
        value_name = "TYPE",
        value_parser = clap::value_parser!(EntryType),
        action = ArgAction::Append,
        num_args = 0..
    )]
    entry_type: Vec<EntryType>,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum EntryType {
    Dir,
    File,
    Link,
}

impl ValueEnum for EntryType {
    fn value_variants<'a>() -> &'a [Self] {
        &[Self::Dir, Self::File, Self::Link]
    }

    fn to_possible_value(&self) -> Option<PossibleValue> {
        match self {
            Self::Dir => Some(PossibleValue::new("d")),
            Self::File => Some(PossibleValue::new("f")),
            Self::Link => Some(PossibleValue::new("l")),
        }
    }
}

fn main() {
    let args = Args::parse();

    if let Err(e) = run(args) {
        eprintln!("{e}");
        std::process::exit(1);
    }
}

fn run(args: Args) -> Result<()> {
    let name_filter =
        |name: &str| args.names.is_empty() || args.names.iter().any(|re| re.is_match(name));
    let type_filter = |file_type: fs::FileType| {
        args.entry_type.is_empty()
            || args.entry_type.iter().any(|t| match t {
                EntryType::Dir => file_type.is_dir(),
                EntryType::File => file_type.is_file(),
                EntryType::Link => file_type.is_symlink(),
            })
    };

    for path in args.path {
        for entry in WalkDir::new(path) {
            match entry {
                Ok(entry) => {
                    if !name_filter(&entry.file_name().to_string_lossy()) {
                        continue;
                    }

                    if !type_filter(entry.file_type()) {
                        continue;
                    }

                    println!("{}", entry.path().display());
                }
                Err(e) => eprintln!("{e}"),
            }
        }
    }

    Ok(())
}
