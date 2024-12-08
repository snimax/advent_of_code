use std::fs;
use std::io::Error;
use std::{ops::Add, ops::Sub};

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

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Pos {
    pub x: i32,
    pub y: i32,
}

impl Add for Pos {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for Pos {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}
