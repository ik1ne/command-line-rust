use anyhow::Result;
use std::fs;
use std::io::{BufRead, BufReader};

pub struct LineCounter {
    pub lines: usize,
    pub bytes: usize,
}

impl LineCounter {
    pub fn new(filename: &str) -> Result<Self> {
        let meta = fs::metadata(filename)?;

        let bytes = meta.len() as usize;
        let lines = BufReader::new(fs::File::open(filename)?).lines().count();

        Ok(Self { lines, bytes })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_lines_bytes() {
        let res = LineCounter::new("tests/inputs/one.txt");
        assert!(res.is_ok());
        let LineCounter { lines, bytes } = res.unwrap();
        assert_eq!(lines, 1);
        assert_eq!(bytes, 24);

        let res = LineCounter::new("tests/inputs/twelve.txt");
        assert!(res.is_ok());
        let LineCounter { lines, bytes } = res.unwrap();
        assert_eq!(lines, 12);
        assert_eq!(bytes, 63);
    }
}
