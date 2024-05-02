use anyhow::{anyhow, Context, Result};
use clap::Parser;
use rand::prelude::*;
use rand::SeedableRng;
use regex::RegexBuilder;

use crate::find_files::find_files;
use crate::fortunes::{read_fortunes, Adage};

mod find_files;
mod fortunes;

#[derive(Debug, Parser)]
pub struct Args {
    #[arg(required(true), value_name = "FILE")]
    sources: Vec<String>,
    #[arg(short = 'm', long)]
    pattern: Option<String>,
    #[arg(short, long)]
    insensitive: bool,
    #[arg(short, long)]
    seed: Option<u64>,
}

fn main() {
    let args = Args::parse();

    if let Err(e) = run(&args) {
        eprintln!("{e}");
        std::process::exit(1);
    }
}

fn run(args: &Args) -> Result<()> {
    let re = args
        .pattern
        .as_ref()
        .map(|p| {
            RegexBuilder::new(p)
                .case_insensitive(args.insensitive)
                .build()
                .map_err(|_| anyhow!(r#"Invalid --pattern "{p}""#))
        })
        .transpose()?;

    let files = find_files(&args.sources)?;
    let adages = read_fortunes(&files)?;

    if adages.is_empty() {
        println!("No fortunes found");
        return Ok(());
    }

    match re {
        None => {
            let adage = pick_random_adage(&adages, args.seed);
            if let Some(adage) = adage {
                println!("{}", adage.text);
            }
        }
        Some(re) => {
            let mut prev_source = None;
            for adage in adages.iter().filter(|adage| re.is_match(&adage.text)) {
                let source = adage.source.context("source is none")?;
                if prev_source != Some(source) {
                    eprintln!("({})\n%", source);
                    prev_source = Some(source);
                }

                println!("{}\n%", adage.text);
            }
        }
    }

    Ok(())
}

fn pick_random_adage<'a, 'b>(adages: &'a [Adage<'b>], seed: Option<u64>) -> Option<&'a Adage<'b>> {
    let mut rng: Box<dyn RngCore> = match seed {
        None => Box::new(thread_rng()),
        Some(seed) => Box::new(StdRng::seed_from_u64(seed)),
    };

    adages.choose(&mut rng)
}

#[cfg(test)]
mod tests {
    use super::{pick_random_adage, Adage};

    #[test]
    fn test_find_files() {} // Same as before

    #[test]
    fn test_read_fortunes() {} // Same as before

    #[test]
    fn test_pick_fortune() {
        // Create a slice of fortunes
        let fortunes = &[
            Adage {
                source: Some("fortunes"),
                text: "You cannot achieve the impossible without \
                      attempting the absurd."
                    .to_string(),
            },
            Adage {
                source: Some("fortunes"),
                text: "Assumption is the mother of all screw-ups.".to_string(),
            },
            Adage {
                source: Some("fortunes"),
                text: "Neckties strangle clear thinking.".to_string(),
            },
        ];

        // Pick a fortune with a seed
        assert_eq!(
            pick_random_adage(fortunes, Some(1)).unwrap().text,
            "Neckties strangle clear thinking.".to_string()
        );
    }
}
