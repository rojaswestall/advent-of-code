use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

/// Reads lines from a file and returns a `Vec<String>`.
/// Returns an `io::Result` to handle potential errors.
pub fn read_lines_from_text(filename: &str) -> io::Result<Vec<String>> {
    let path = Path::new(filename);
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    reader.lines().collect()
}