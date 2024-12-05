use std::fs;
use std::io::Error;

pub fn parse_file(file: &str) -> Result<String, Error> {
    fs::read_to_string(file)
}

pub fn parse_lines(s: &str) -> Vec<String> {
    let mut lines: Vec<String> = Vec::new();

    for line in s.lines() {
        lines.push(line.to_string());
    }

    lines
}
