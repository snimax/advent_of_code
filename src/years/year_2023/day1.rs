use crate::years::AdventDay;

pub struct Day1 {}

impl AdventDay for Day1 {
    fn solve(&self) {
        let lines = self.get_input();
        println!("Part1 solution: {}", part1(&lines));
        println!("Part2 solution: {}", part2(&lines));
    }

    fn get_input_path(&self) -> &str {
        "Inputs/2023/day1.txt"
    }
}

fn get_string_num_matches(line: &str, pattern: &str, char: char) -> Vec<(usize, char)> {
    let mut res = Vec::new();
    for (idx, _) in line.match_indices(pattern) {
        res.push((idx, char));
    }
    res
}

fn get_number_chars(line: &str, allow_number_names: bool) -> Vec<char> {
    let mut digit_matches: Vec<(usize, char)> = line
        .chars()
        .enumerate()
        .filter(|(_, c)| c.is_ascii_digit())
        .collect();

    if allow_number_names {
        digit_matches.append(&mut get_string_num_matches(line, "one", '1'));
        digit_matches.append(&mut get_string_num_matches(line, "two", '2'));
        digit_matches.append(&mut get_string_num_matches(line, "three", '3'));
        digit_matches.append(&mut get_string_num_matches(line, "four", '4'));
        digit_matches.append(&mut get_string_num_matches(line, "five", '5'));
        digit_matches.append(&mut get_string_num_matches(line, "six", '6'));
        digit_matches.append(&mut get_string_num_matches(line, "seven", '7'));
        digit_matches.append(&mut get_string_num_matches(line, "eight", '8'));
        digit_matches.append(&mut get_string_num_matches(line, "nine", '9'));

        digit_matches.sort_by_key(|k| k.0);
    }

    digit_matches.iter().map(|(_, c)| *c).collect()
}

fn create_calibration_num(number_chars: &[char]) -> u32 {
    let mut new_num = String::new();
    new_num.push(*number_chars.first().unwrap());
    new_num.push(*number_chars.last().unwrap());

    new_num.parse::<u32>().unwrap_or(0)
}

fn part1(lines: &[String]) -> u32 {
    lines
        .iter()
        .map(|line| {
            let numbers = get_number_chars(line, false);
            create_calibration_num(&numbers)
        })
        .sum()
}

fn part2(lines: &[String]) -> u32 {
    lines
        .iter()
        .map(|line| {
            let numbers = get_number_chars(line, true);
            create_calibration_num(&numbers)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::library::*;

    fn get_lines_part_1() -> Vec<String> {
        let input = r#"1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet"#;

        parse_lines(&input)
    }

    fn get_lines_part_2() -> Vec<String> {
        let input = r#"two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen"#;

        parse_lines(&input)
    }

    #[test]
    fn test_part1() -> Result<(), String> {
        let lines = get_lines_part_1();
        assert_eq!(part1(&lines), 142);

        Ok(())
    }

    #[test]
    fn test_part2() -> Result<(), String> {
        let lines = get_lines_part_2();
        assert_eq!(part2(&lines), 281);

        Ok(())
    }
}
