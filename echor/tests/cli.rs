use anyhow::Result;
use assert_cmd::Command;

const BINARY: &str = "echor";

#[test]
fn dies_no_args() -> Result<()> {
    let mut cmd = Command::cargo_bin(BINARY)?;
    cmd.assert()
        .failure()
        .stderr(predicates::str::contains("Usage"));

    Ok(())
}

fn run(args: &[&str], expected_file: &str) -> Result<()> {
    let expected = std::fs::read_to_string(expected_file)?;
    let output = Command::cargo_bin(BINARY)?.args(args).output()?;

    let stdout = String::from_utf8(output.stdout)?;

    assert_eq!(stdout, expected);

    Ok(())
}

#[test]
fn hello1() -> Result<()> {
    run(&["Hello there"], "tests/expected/hello1.txt")
}

#[test]
fn hello2() -> Result<()> {
    run(&["Hello", "there"], "tests/expected/hello2.txt")
}

#[test]
fn hello1_no_newline() -> Result<()> {
    run(&["Hello  there", "-n"], "tests/expected/hello1.n.txt")
}

#[test]
fn hello2_no_newline() -> Result<()> {
    run(&["-n", "Hello", "there"], "tests/expected/hello2.n.txt")
}
