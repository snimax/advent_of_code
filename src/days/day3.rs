use super::{parse_file, parse_lines};
use regex::Regex;
use std::num::ParseIntError;

pub fn solve() {
    if let Ok(line_string) = parse_file("Inputs/day3.txt") {
        let lines = parse_lines(&line_string);
        println!("Part1 solution: {}", part1(&lines));
        println!("Part2 solution: {}", part2(&lines));
    } else {
        println!("Could not parse file");
    }
}

fn parse_and_mul(a: &str, b: &str) -> Result<i32, ParseIntError> {
    let a = a.parse::<i32>()?;
    let b = b.parse::<i32>()?;
    Ok(a * b)
}

fn extract_and_mul(line: &str) -> i32 {
    let regexpr = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    let mut result = 0;
    for (_, [a, b]) in regexpr.captures_iter(line).map(|c| c.extract()) {
        result += parse_and_mul(a, b).unwrap_or(0);
    }
    result
}

fn part1(lines: &[String]) -> i32 {
    let mut result = 0;

    for line in lines.iter() {
        result += extract_and_mul(line);
    }
    result
}

fn part2(lines: &[String]) -> i32 {
    let regexpr =
        Regex::new(r"(?<mul>mul\(\d+,\d+\))|(?<enable>do\(\))|(?<disable>don't\(\))").unwrap();
    let mut result = 0;
    let mut enabled = true;

    for line in lines.iter() {
        for capture in regexpr.captures_iter(line) {
            if let Some(c) = capture.name("mul") {
                if enabled {
                    result += extract_and_mul(c.as_str());
                }
            } else if capture.name("enable").is_some() {
                enabled = true;
            } else if capture.name("disable").is_some() {
                enabled = false;
            }
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() -> Result<(), String> {
        let lines = vec![
            r"xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))".to_string(),
        ];
        assert_eq!(part1(&lines), 161);

        Ok(())
    }

    #[test]
    fn test_part2() -> Result<(), String> {
        let lines = vec![
            r"xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"
                .to_string(),
        ];
        assert_eq!(part2(&lines), 48);

        Ok(())
    }
}
