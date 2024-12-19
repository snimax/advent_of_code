use advent_of_code_2024::{parse_file, parse_lines};
use std::thread;
use std::time::Instant;

pub fn solve() {
    if let Ok(line_string) = parse_file("Inputs/day19.txt") {
        let lines = parse_lines(&line_string);
        let (available_patterns, patterns_to_make) = parse_towels(&lines);
        let now = Instant::now();
        println!(
            "Part1 solution: {}",
            part1(&available_patterns, &patterns_to_make)
        );
        let elapsed = now.elapsed();
        println!("{elapsed:?}");
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

const COLORS: [Color; 5] = [
    Color::White,
    Color::Blue,
    Color::Black,
    Color::Red,
    Color::Green,
];

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

// fn make_pattern(pattern: &[Color], available_patterns: &[Towel]) -> Option<()> {
//     if pattern.len() == 0 {
//         return Some(());
//     }

//     for available_pattern in available_patterns {
//         let pattern_len = available_pattern.len();
//         if pattern.len() < pattern_len {
//             continue;
//         }
//         if pattern[..pattern_len] == *available_pattern {
//             let res = make_pattern(&pattern[pattern_len..], available_patterns);
//             if res.is_some() {
//                 return res;
//             }
//         }
//     }

//     None
// }

fn make_pattern(idx: usize, pattern: &[Color], available_patterns: &[Towel]) -> Option<()> {
    if idx == pattern.len() {
        return Some(());
    }

    let left_to_match = pattern.len() - idx;
    for available_pattern in available_patterns {
        let pattern_len = available_pattern.len();
        if left_to_match < pattern_len {
            continue;
        }

        if pattern[idx..(idx + pattern_len)] == *available_pattern {
            let res = make_pattern(idx + pattern_len, pattern, available_patterns);
            if res.is_some() {
                return res;
            }
        }
    }

    None
}

fn part1(available_patterns: &[Towel], patterns_to_make: &[Towel]) -> usize {
    let len = patterns_to_make.len();

    let single_colors_available: Towel = available_patterns
        .iter()
        .filter(|a| a.len() == 1)
        .map(|c| c[0])
        .collect::<Towel>();

    let look_for_patterns_with_colors: Towel = COLORS
        .iter()
        .filter(|c| !single_colors_available.contains(c))
        .cloned()
        .collect();

    let patterns_to_test: Vec<Towel> = available_patterns
        .iter()
        .filter(|pattern| {
            look_for_patterns_with_colors
                .iter()
                .any(|c| pattern.contains(c))
                || pattern.len() == 1
        })
        .cloned()
        .collect();

    // let patterns_to_test = available_patterns;

    patterns_to_make
        .iter()
        .enumerate()
        .filter_map(|(idx, pattern)| {
            println!("{idx}/{len}");
            make_pattern(0, pattern, &patterns_to_test)
        })
        .count()
}

fn part2(available_patterns: &[Towel], patterns_to_make: &[Towel]) -> usize {
    0
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
        assert_eq!(part2(&available_patterns, &patterns_to_make), 0);

        Ok(())
    }
}
