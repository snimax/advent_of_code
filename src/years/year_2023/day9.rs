use crate::years::AdventDay;
pub struct Day9 {}

impl AdventDay for Day9 {
    fn solve(&self) {
        let lines = self.get_input();
        let histories = parse_input(&lines);
        println!("Part1 solution: {}", part1(&histories));
        println!("Part2 solution: {}", part2(&histories));
    }

    fn get_input_path(&self) -> &str {
        "Inputs/2023/day9.txt"
    }
}

type History = Vec<i32>;

fn parse_input(lines: &[String]) -> Vec<History> {
    lines
        .iter()
        .map(|line| {
            line.split_ascii_whitespace()
                .filter_map(|str| str.parse::<i32>().ok())
                .collect()
        })
        .collect()
}

fn get_differences(history: &History) -> History {
    history
        .iter()
        .skip(1)
        .zip(history)
        .map(|(a, b)| a - b)
        .collect()
}

fn extrapolate_history_val(history: &History) -> i32 {
    if history.iter().all(|point| *point == history[0]) {
        return history[0];
    }

    extrapolate_history_val(&get_differences(history)) + history.last().unwrap()
}

fn part1(histories: &[History]) -> i32 {
    histories.iter().map(extrapolate_history_val).sum()
}

fn part2(histories: &[History]) -> i32 {
    histories
        .iter()
        .map(|history| {
            let new_history: Vec<i32> = history.iter().rev().cloned().collect();
            extrapolate_history_val(&new_history)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::library::*;

    fn get_lines() -> Vec<History> {
        let input = r#"0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45"#;

        parse_input(&parse_lines(&input))
    }

    #[test]
    fn test_part1() -> Result<(), String> {
        let histories = get_lines();
        assert_eq!(part1(&histories), 114);

        Ok(())
    }

    #[test]
    fn test_part2() -> Result<(), String> {
        let histories = get_lines();
        assert_eq!(part2(&histories), 2);

        Ok(())
    }
}
