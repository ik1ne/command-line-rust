use std::fs;

use anyhow::{anyhow, Result};
use walkdir::WalkDir;

pub fn find_files(paths: &[String], recursive: bool) -> Vec<Result<String>> {
    let mut results = vec![];

    for path in paths {
        if path == "-" {
            results.push(Ok(path.to_string()));
            continue;
        }

        append_files(&mut results, path, recursive);
    }

    results
}

fn append_files(results: &mut Vec<Result<String>>, path: &str, recursive: bool) {
    match fs::metadata(path) {
        Ok(metadata) => {
            if metadata.is_dir() {
                if recursive {
                    for subpath in WalkDir::new(path) {
                        match subpath {
                            Ok(path) => {
                                if path.path().is_file() {
                                    append_files(
                                        results,
                                        &path.path().to_string_lossy(),
                                        recursive,
                                    );
                                }
                            }
                            Err(e) => results.push(Err(anyhow!("{path}: {e}"))),
                        };
                    }
                } else {
                    results.push(Err(anyhow!("{path} is a directory")));
                }
            } else {
                results.push(Ok(path.to_string()));
            }
        }

        Err(e) => {
            results.push(Err(anyhow!("{path}: {e}")));
        }
    }
}

#[cfg(test)]
mod tests {
    use rand::{distributions::Alphanumeric, Rng};

    use super::find_files;

    #[test]
    fn test_find_files() {
        // Verify that the function finds a file known to exist
        let files = find_files(&["./tests/inputs/fox.txt".to_string()], false);
        assert_eq!(files.len(), 1);
        assert_eq!(files[0].as_ref().unwrap(), "./tests/inputs/fox.txt");

        // The function should reject a directory without the recursive option
        let files = find_files(&["./tests/inputs".to_string()], false);
        assert_eq!(files.len(), 1);
        if let Err(e) = &files[0] {
            assert_eq!(e.to_string(), "./tests/inputs is a directory");
        }

        // Verify the function recurses to find four files in the directory
        let res = find_files(&["./tests/inputs".to_string()], true);
        let mut files: Vec<String> = res
            .iter()
            .map(|r| r.as_ref().unwrap().replace("\\", "/"))
            .collect();
        files.sort();
        assert_eq!(files.len(), 4);
        assert_eq!(
            files,
            vec![
                "./tests/inputs/bustle.txt",
                "./tests/inputs/empty.txt",
                "./tests/inputs/fox.txt",
                "./tests/inputs/nobody.txt",
            ]
        );

        // Generate a random string to represent a nonexistent file
        let bad: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(7)
            .map(char::from)
            .collect();

        // Verify that the function returns the bad file as an error
        let files = find_files(&[bad], false);
        assert_eq!(files.len(), 1);
        assert!(files[0].is_err());
    }
}
