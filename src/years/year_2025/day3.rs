use crate::years::AdventDay;
use std::collections::HashMap;

pub struct Day3 {}

impl AdventDay for Day3 {
    fn solve(&self) {
        let lines = self.get_input();
        println!("Part1 solution: {}", part1(&lines));
        println!("Part2 solution: {}", part2(&lines)); // 173848577117259 too low
    }

    fn get_input_path(&self) -> &str {
        "Inputs/2025/day3.txt"
    }
}

type Bank = Vec<u32>;

fn parse_banks(lines: &[String]) -> Vec<Bank> {
    lines
        .iter()
        .map(|line| {
            line.chars()
                .filter_map(|c| c.to_digit(10))
                .collect::<Bank>()
        })
        .collect()
}

fn get_largest_joltage(bank: &Bank) -> u32 {
    let mut largest_joltage = 0;
    bank.iter().enumerate().for_each(|(idx, battery)| {
        bank.iter().skip(idx + 1).for_each(|battery2| {
            largest_joltage = u32::max(largest_joltage, battery * 10 + battery2);
        });
    });
    largest_joltage
}

fn part1(lines: &[String]) -> u32 {
    let banks = parse_banks(lines);
    banks.iter().map(get_largest_joltage).sum()
}

fn get_largest_joltage2<'a>(
    bank: &'a [u32],
    batteries_to_turn_on: usize,
    memoization: &mut HashMap<(&'a [u32], usize), u64>,
) -> u64 {
    let key = (bank, batteries_to_turn_on);
    if let Some(val) = memoization.get(&key) {
        return *val;
    }

    if bank.len() < batteries_to_turn_on {
        return 0;
    }

    let battery_power = bank[0] as u64 * 10_u64.pow(batteries_to_turn_on as u32 - 1);
    let rest_of_bank = &bank[1..];
    let battery_choosen = match batteries_to_turn_on {
        1 => battery_power,
        _ => {
            battery_power
                + get_largest_joltage2(rest_of_bank, batteries_to_turn_on - 1, memoization)
        }
    };

    let battery_skipped = get_largest_joltage2(rest_of_bank, batteries_to_turn_on, memoization);

    let result = u64::max(battery_choosen, battery_skipped);
    memoization.insert(key, result);
    result
}

fn part2(lines: &[String]) -> u64 {
    let banks = parse_banks(lines);
    banks
        .iter()
        .map(|bank| {
            let mut memoization = HashMap::new();
            get_largest_joltage2(bank, 12, &mut memoization)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::library::*;

    fn get_lines() -> Vec<String> {
        let input = r#"987654321111111
811111111111119
234234234234278
818181911112111"#;

        parse_lines(&input)
    }

    #[test]
    fn test_part1() -> Result<(), String> {
        let lines = get_lines();
        assert_eq!(part1(&lines), 357);

        Ok(())
    }

    #[test]
    fn test_part2() -> Result<(), String> {
        let lines = get_lines();
        assert_eq!(part2(&lines), 3121910778619);

        Ok(())
    }
}
