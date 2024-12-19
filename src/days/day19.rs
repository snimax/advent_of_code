use advent_of_code_2024::{parse_file, parse_lines};
use std::collections::HashMap;

pub fn solve() {
    if let Ok(line_string) = parse_file("Inputs/day19.txt") {
        let lines = parse_lines(&line_string);
        let (available_patterns, patterns_to_make) = parse_towels(&lines);
        println!(
            "Part1 solution: {}",
            part1(&available_patterns, &patterns_to_make)
        );
        println!(
            "Part2 solution: {}",
            part2(&available_patterns, &patterns_to_make)
        );
    } else {
        println!("Could not parse file");
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Color {
    White,
    Blue,
    Black,
    Red,
    Green,
}

type Towel = Vec<Color>;

fn parse_towel(str: &str) -> Towel {
    str.chars()
        .map(|c| match c {
            'w' => Color::White,
            'u' => Color::Blue,
            'b' => Color::Black,
            'r' => Color::Red,
            'g' => Color::Green,
            c => panic!("Got unexpected color '{c}'"),
        })
        .collect::<Towel>()
}

fn parse_towels(lines: &[String]) -> (Vec<Towel>, Vec<Towel>) {
    let available_towel_patterns = lines[0].split(", ").map(parse_towel).collect();

    let patterns_to_make = lines.iter().skip(2).map(|s| parse_towel(s)).collect();

    (available_towel_patterns, patterns_to_make)
}

fn part1(available_patterns: &[Towel], patterns_to_make: &[Towel]) -> usize {
    patterns_to_make.iter().fold(0, |acc, pattern| {
        let mut memoization = HashMap::new();
        acc + find_possible_patterns(0, pattern, available_patterns, &mut memoization, false)
    })
}

fn find_possible_patterns(
    idx: usize,
    pattern: &[Color],
    available_patterns: &[Towel],
    memoization: &mut HashMap<usize, usize>,
    find_all: bool,
) -> usize {
    if let Some(val) = memoization.get(&idx) {
        return *val;
    }

    if idx == pattern.len() {
        return 1;
    }

    let mut result_count = 0;

    let left_to_match = pattern.len() - idx;
    for available_pattern in available_patterns {
        let pattern_len = available_pattern.len();
        if left_to_match < pattern_len {
            continue;
        }

        if pattern[idx..(idx + pattern_len)] == *available_pattern {
            result_count += find_possible_patterns(
                idx + pattern_len,
                pattern,
                available_patterns,
                memoization,
                find_all,
            );
            if !find_all && result_count == 1 {
                return result_count;
            }
        }
    }

    if let Some(val) = memoization.get_mut(&idx) {
        *val += result_count;
    } else {
        memoization.insert(idx, result_count);
    }

    result_count
}

fn part2(available_patterns: &[Towel], patterns_to_make: &[Towel]) -> usize {
    patterns_to_make.iter().fold(0, |acc, pattern| {
        let mut memoization = HashMap::new();
        acc + find_possible_patterns(0, pattern, available_patterns, &mut memoization, true)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_input() -> (Vec<Towel>, Vec<Towel>) {
        let input = r#"r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb"#;

        let lines = parse_lines(&input);
        parse_towels(&lines)
    }

    #[test]
    fn test_part1() -> Result<(), String> {
        let (available_patterns, patterns_to_make) = get_input();
        assert_eq!(part1(&available_patterns, &patterns_to_make), 6);

        Ok(())
    }

    #[test]
    fn test_part2() -> Result<(), String> {
        let (available_patterns, patterns_to_make) = get_input();
        assert_eq!(part2(&available_patterns, &patterns_to_make), 16);

        Ok(())
    }
}
