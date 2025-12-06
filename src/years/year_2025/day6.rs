use crate::years::AdventDay;

pub struct Day6 {}

impl AdventDay for Day6 {
    fn solve(&self) {
        let lines = self.get_input();
        println!("Part1 solution: {}", part1(&lines));
        println!("Part2 solution: {}", part2(&lines));
    }

    fn get_input_path(&self) -> &str {
        "Inputs/2025/day6.txt"
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Operation {
    Multiply,
    Add,
}

#[derive(Debug)]
struct Problem {
    numbers: Vec<u64>,
    op: Operation,
}

fn parse_strings(lines: &[String]) -> (Vec<Vec<u64>>, Vec<Operation>) {
    let num_elements = if let Some(line) = lines.first() {
        line.split_ascii_whitespace().count()
    } else {
        0
    };

    let mut numbers = vec![Vec::new(); num_elements];
    let mut operations = vec![Operation::Multiply; num_elements];
    for line in lines {
        line.split_ascii_whitespace()
            .enumerate()
            .for_each(|(idx, num)| {
                if let Ok(num) = num.parse() {
                    numbers[idx].push(num);
                } else {
                    match num {
                        "*" => operations[idx] = Operation::Multiply,
                        "+" => operations[idx] = Operation::Add,
                        _ => panic!("Couldn't parse '{num}' to either a number or operation"),
                    }
                }
            });
    }
    (numbers, operations)
}

fn zip_problems(numbers: &[Vec<u64>], operations: &[Operation]) -> Vec<Problem> {
    numbers
        .iter()
        .zip(operations)
        .map(|(numbers, op)| Problem {
            numbers: numbers.clone(),
            op: *op,
        })
        .collect()
}

fn parse_homework(lines: &[String]) -> Vec<Problem> {
    let (numbers, operations) = parse_strings(lines);
    zip_problems(&numbers, &operations)
}

fn solve_homework(homework: &[Problem]) -> u64 {
    homework
        .iter()
        .map(|Problem { numbers, op }| match op {
            Operation::Multiply => numbers.iter().product::<u64>(),
            Operation::Add => numbers.iter().sum(),
        })
        .sum()
}

fn part1(lines: &[String]) -> u64 {
    let homework = parse_homework(lines);
    solve_homework(&homework)
}

fn parse_transposed_howework(lines: &[String]) -> Vec<Problem> {
    let mut transposed = vec![String::new(); lines[0].len() + 1];
    let mut operations = Vec::new();
    for line in lines {
        for (idx, char) in line.chars().enumerate() {
            match char {
                '*' => operations.push(Operation::Multiply),
                '+' => operations.push(Operation::Add),
                c => transposed[idx].push(c),
            }
        }
    }

    let mut numbers_str = vec![String::new(); 1];
    let mut idx = 0;
    for str in transposed {
        if str.trim().is_empty() {
            idx += 1;
            numbers_str.push(String::new());
        }
        numbers_str[idx].push(' '); // Making sure there is atleast one space between numbers
        numbers_str[idx].push_str(&str);
    }

    let numbers: Vec<Vec<u64>> = numbers_str
        .iter()
        .map(|str| {
            str.split_ascii_whitespace()
                .map(|n| n.parse().unwrap_or(0))
                .collect()
        })
        .collect();

    zip_problems(&numbers, &operations)
}

fn part2(lines: &[String]) -> u64 {
    let homework = parse_transposed_howework(lines);
    solve_homework(&homework)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::library::*;

    fn get_lines() -> Vec<String> {
        let input = r#"123 328  51 64
 45 64  387 23
  6 98  215 314
*   +   *   +"#;

        parse_lines(&input)
    }

    #[test]
    fn test_part1() -> Result<(), String> {
        let lines = get_lines();
        assert_eq!(part1(&lines), 4277556);

        Ok(())
    }

    #[test]
    fn test_part2() -> Result<(), String> {
        let lines = get_lines();
        assert_eq!(part2(&lines), 3263827);

        Ok(())
    }
}
