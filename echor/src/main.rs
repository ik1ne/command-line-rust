use clap::Parser;

#[derive(Debug, Parser)]
#[command(author, version, about)]
/// Rust version of `echo
struct Args {
    /// Input text
    #[arg(required = true)]
    text: Vec<String>,

    /// Do not print newline
    #[arg(short = 'n')]
    omit_newline: bool,
}

fn main() {
    let args = Args::parse();

    match args.omit_newline {
        true => print!("{}", args.text.join(" ")),
        false => println!("{}", args.text.join(" ")),
    };
}
