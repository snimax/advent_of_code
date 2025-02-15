pub mod pos;
pub mod dir;
pub mod map;

use std::fs;
use std::io::Error;
use std::ops::{Add, Div, Mul, Sub};

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

pub trait NumericOps:
    Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
    + PartialEq
    + PartialOrd
    + Copy
{
}
impl<T> NumericOps for T where
    T: Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + Div<Output = T>
        + PartialEq
        + PartialOrd
        + Copy
{
}

#[derive(PartialEq, Clone, Debug)]
pub struct Equation<T: NumericOps> {
    pub x: T,
    pub y: T,
    pub ans: T,
}

pub fn cramers_rule<T: NumericOps>(eq1: &Equation<T>, eq2: &Equation<T>) -> (T, T) {
    let denominator = eq1.x * eq2.y - eq2.x * eq1.y;
    let x_nominator = eq1.ans * eq2.y - eq1.y * eq2.ans;
    let y_nominator = eq1.x * eq2.ans - eq1.ans * eq2.x;

    (x_nominator / denominator, y_nominator / denominator)
}
