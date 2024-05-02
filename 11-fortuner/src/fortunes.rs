use anyhow::Result;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

pub struct Adage<'a> {
    pub source: Option<&'a str>,
    pub text: String,
}

pub fn read_fortunes(paths: &[PathBuf]) -> Result<Vec<Adage>> {
    let mut result = vec![];

    for path in paths {
        let adages = read_fortune(path)?;
        result.extend(adages);
    }

    Ok(result)
}

fn read_fortune(path: &PathBuf) -> Result<Vec<Adage>> {
    let mut file = BufReader::new(File::open(path)?);
    let file_name = path.file_name().and_then(|s| s.to_str());

    let mut result = vec![];

    let mut line = vec![];
    loop {
        let bytes_read = file.read_until(b'%', &mut line)?;
        if bytes_read == 0 {
            break;
        }

        let text = String::from_utf8_lossy(&line);
        // remove trailing %
        let text = &text[..text.len() - 1];
        let text = text.trim().to_string();

        if !text.is_empty() {
            result.push(Adage {
                source: file_name,
                text,
            });
        }

        line.clear();
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::read_fortunes;
    use std::path::PathBuf;

    #[test]
    fn test_find_files() {} // Same as before

    #[test]
    fn test_read_fortunes() {
        // One input file
        let input = [PathBuf::from("./tests/inputs/jokes")];
        let res = read_fortunes(&input);
        assert!(res.is_ok());

        if let Ok(adages) = res {
            // Correct number and sorting
            assert_eq!(adages.len(), 6);
            assert_eq!(
                adages.first().unwrap().text,
                "Q. What do you call a head of lettuce in a shirt and tie?\n\
                A. Collared greens."
            );
            assert_eq!(
                adages.last().unwrap().text,
                "Q: What do you call a deer wearing an eye patch?\n\
                A: A bad idea (bad-eye deer)."
            );
        }

        // Multiple input files
        let input = [
            PathBuf::from("./tests/inputs/jokes"),
            PathBuf::from("./tests/inputs/quotes"),
        ];
        let res = read_fortunes(&input);
        assert!(res.is_ok());
        assert_eq!(res.unwrap().len(), 11);
    }
}
