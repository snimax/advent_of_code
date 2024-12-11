use advent_of_code_2024::parse_file;
use std::collections::HashMap;

pub fn solve() {
    if let Ok(line_string) = parse_file("Inputs/day11.txt") {
        let stones = parse_line(&line_string);
        println!("Part1 solution: {}", part1(&stones));
        println!("Part2 solution: {}", part2(&stones));
    } else {
        println!("Could not parse file");
    }
}

fn parse_line(line: &str) -> Vec<usize> {
    line.split_ascii_whitespace()
        .map(|s| s.parse::<usize>().unwrap())
        .collect()
}

fn part1(stones: &[usize]) -> usize {
    let mut seen_values = HashMap::new();
    stones
        .iter()
        .fold(0, |acc, s| acc + recurse(*s, 25, &mut seen_values))
}

fn number_of_digits(n: usize) -> usize {
    if n == 0 {
        1
    } else {
        (n as f64).log10().floor() as usize + 1
    }
}

fn split_usize(n: usize) -> (usize, usize) {
    let digits = n.to_string();
    let len = digits.len();
    let mid = len / 2;
    let (first_half, second_half) = digits.split_at(mid);
    (
        first_half.parse::<usize>().unwrap(),
        second_half.parse::<usize>().unwrap(),
    )
}

fn recurse(stone: usize, depth: usize, seen_values: &mut HashMap<(usize, usize), usize>) -> usize {
    let key = (stone, depth);

    let cached_val = { seen_values.get(&key) };

    if let Some(cached_val) = cached_val {
        return *cached_val;
    }

    if depth == 0 {
        seen_values.insert(key, 1);
        return 1;
    }

    if stone == 0 {
        let res = recurse(1, depth - 1, seen_values);
        seen_values.insert(key, res);
        return res;
    }

    let num_digits = number_of_digits(stone);

    if num_digits % 2 == 0 {
        let (a, b) = split_usize(stone);
        let res = recurse(a, depth - 1, seen_values) + recurse(b, depth - 1, seen_values);
        seen_values.insert(key, res);
        return res;
    }

    let res = recurse(stone * 2024, depth - 1, seen_values);
    seen_values.insert(key, res);
    res
}

fn part2(stones: &[usize]) -> usize {
    let mut seen_values = HashMap::new();
    stones
        .iter()
        .fold(0, |acc, s| acc + recurse(*s, 75, &mut seen_values))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_lines() -> Vec<usize> {
        let input = r#"125 17"#;
        parse_line(&input)
    }

    #[test]
    fn test_part1() -> Result<(), String> {
        let lines = get_lines();
        assert_eq!(part1(&lines), 55312);

        Ok(())
    }
}
