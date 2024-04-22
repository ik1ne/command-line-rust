use clap::builder::PossibleValue;
use clap::{ArgAction, Parser, ValueEnum};
use regex::Regex;

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
    dbg!(args);
}
