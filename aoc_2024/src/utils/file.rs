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

pub fn get_string_matrix_from_text_input(filename: &str) -> Vec<Vec<String>> {
    let lines = match read_lines_from_text(filename) {
        Ok(lines) => lines,
        Err(e) => {
            panic!("Error reading file: {}", e);
        }
    };

    let mut matrix = Vec::new();
    for line in lines {
        let mut row = Vec::new();
        for ch in line.chars() {
            row.push(ch.to_string());
        }
        matrix.push(row)
    }

    return matrix;
}

pub fn get_i32_matrix_from_text_input(filename: &str) -> Vec<Vec<i32>> {
    let lines = match read_lines_from_text(filename) {
        Ok(lines) => lines,
        Err(e) => {
            panic!("Error reading file: {}", e);
        }
    };

    let mut matrix = Vec::new();
    for line in lines {
        let mut row = Vec::new();
        for ch in line.chars() {
            let parsed = ch.to_digit(10).map(|d| d as i32).expect("Error parsing input into i32");
            row.push(parsed);
        }
        matrix.push(row)
    }

    return matrix;
}
