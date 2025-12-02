use crate::years::AdventDay;

pub struct Day2 {}

impl AdventDay for Day2 {
    fn solve(&self) {
        let lines = self.get_input();
        println!("Part1 solution: {}", part1(&lines));
        println!("Part2 solution: {}", part2(&lines));
    }

    fn get_input_path(&self) -> &str {
        "Inputs/2025/day2.txt"
    }
}

struct Range {
    start: u64,
    end: u64,
}

fn parse_ranges(lines: &[String]) -> Vec<Range> {
    let mut ranges = Vec::new();
    for line in lines {
        for range_str in line.split(',') {
            let mut range_parts = range_str.split('-');

            let start = range_parts.next();
            let end = range_parts.next();

            if start.is_none() || end.is_none() {
                panic!("Could not parse range out of {}", range_str);
            }

            if let Ok(start) = start.unwrap().parse()
                && let Ok(end) = end.unwrap().parse()
            {
                ranges.push(Range { start, end });
                continue;
            }
            panic!("Could not parse range from {range_str}");
        }
    }
    ranges
}

fn is_mirrored_number(id: &u64) -> bool {
    // Can't mirror a number if it only consists of a single digit
    if *id < 10 {
        return false;
    }

    let str = id.to_string();
    let str_len = str.len();
    if str_len % 2 == 1 {
        return false;
    }

    let parts = str.split_at(str_len / 2);

    parts.0 == parts.1
}

fn number_contains_repeated_sequence_recursive(pattern: &str, tail: &str) -> bool {
    if tail.is_empty() {
        return true;
    }
    if pattern.len() > tail.len() {
        return false;
    }
    *pattern == tail[0..pattern.len()]
        && number_contains_repeated_sequence_recursive(pattern, &tail[pattern.len()..])
}

fn number_contains_repeated_sequence(id: &u64) -> bool {
    // Can't mirror a number if it only consists of a single digit
    if *id < 10 {
        return false;
    }

    let str = id.to_string();
    let str_len = str.len();

    for split_idx in 1..=(str_len / 2) {
        let (pattern, tail) = str.split_at(split_idx);
        if number_contains_repeated_sequence_recursive(pattern, tail) {
            return true;
        }
    }

    false
}

fn get_invalid_ids<F>(range: &Range, validation_function: F) -> Vec<u64>
where
    F: Fn(&u64) -> bool,
{
    (range.start..=range.end)
        .filter(|r| validation_function(r))
        .collect()
}

fn part1(lines: &[String]) -> u64 {
    let ranges = parse_ranges(lines);
    ranges
        .iter()
        .map(|range| {
            get_invalid_ids(range, is_mirrored_number)
                .iter()
                .sum::<u64>()
        })
        .sum()
}

fn part2(lines: &[String]) -> u64 {
    let ranges = parse_ranges(lines);
    ranges
        .iter()
        .map(|range| {
            get_invalid_ids(range, number_contains_repeated_sequence)
                .iter()
                .sum::<u64>()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::library::*;

    fn get_lines() -> Vec<String> {
        let input = r#"11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124"#;

        parse_lines(&input)
    }

    #[test]
    fn test_part1() -> Result<(), String> {
        let lines = get_lines();
        assert_eq!(part1(&lines), 1227775554);

        Ok(())
    }

    #[test]
    fn test_part2() -> Result<(), String> {
        let lines = get_lines();
        assert_eq!(part2(&lines), 4174379265);

        Ok(())
    }
}
