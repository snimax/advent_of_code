use crate::years::AdventDay;

pub struct Day7 {}

impl AdventDay for Day7 {
    fn solve(&self) {
        let lines = self.get_input();
        let equations = parse_equations(&lines);
        println!("Part1 solution: {}", part1(&equations));
        println!("Part2 solution: {}", part2(&equations));
    }

    fn get_input_path(&self) -> &str {
        "Inputs/2024/day7.txt"
    }
}

fn parse_equation(line: &str) -> (usize, Vec<usize>) {
    let mut split = line.split(':');

    let answer = split.next().unwrap().parse::<usize>().unwrap();

    let mut numbers = Vec::new();
    if let Some(numbers_string) = split.next() {
        for number_string in numbers_string.split_ascii_whitespace() {
            numbers.push(number_string.parse::<usize>().unwrap());
        }
    }
    (answer, numbers)
}

fn parse_equations(lines: &[String]) -> Vec<(usize, Vec<usize>)> {
    lines.iter().map(|line| parse_equation(line)).collect()
}

fn try_solve_equation_recursive(
    answer: &usize,
    running_val: usize,
    numbers: &[usize],
    allow_concatenation: bool,
) -> Option<usize> {
    if running_val > *answer {
        return None;
    }

    if numbers.is_empty() {
        return Some(running_val);
    }

    let first_num = numbers[0];
    let rest = &numbers[1..];
    let mut results = vec![
        try_solve_equation_recursive(answer, running_val * first_num, rest, allow_concatenation),
        try_solve_equation_recursive(answer, running_val + first_num, rest, allow_concatenation),
    ];

    if allow_concatenation {
        let mut str = running_val.to_string();
        str.push_str(&first_num.to_string());
        let concat = str.parse::<usize>().unwrap();
        results.push(try_solve_equation_recursive(
            answer,
            concat,
            rest,
            allow_concatenation,
        ));
    }

    results
        .iter()
        .map(|val| val.and_then(|val| if val <= *answer { Some(val) } else { None }))
        .filter(|val| val.is_some())
        .reduce(|acc, f| Some(usize::max(acc.unwrap(), f.unwrap())))?
}

fn try_solve_equation(
    answer: &usize,
    numbers: &[usize],
    allow_concatenation: bool,
) -> Option<usize> {
    let first_num = numbers[0];
    let rest = &numbers[1..];

    if let Some(ans) = try_solve_equation_recursive(answer, first_num, rest, allow_concatenation) {
        if ans == *answer {
            return Some(ans);
        }
    }

    None
}

fn part1(equations: &[(usize, Vec<usize>)]) -> usize {
    equations
        .iter()
        .map(|(ans, nums)| try_solve_equation(ans, nums, false))
        .filter(|ans| ans.is_some())
        .fold(0, |acc, result| acc + result.unwrap())
}

fn part2(equations: &[(usize, Vec<usize>)]) -> usize {
    equations
        .iter()
        .map(|(ans, nums)| try_solve_equation(ans, nums, true))
        .filter(|ans| ans.is_some())
        .fold(0, |acc, result| acc + result.unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::library::*;

    fn get_equations() -> Vec<(usize, Vec<usize>)> {
        let input = r#"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20"#;

        let lines = parse_lines(&input);
        parse_equations(&lines)
    }

    #[test]
    fn test_part1() -> Result<(), String> {
        let lines = get_equations();
        assert_eq!(part1(&lines), 3749);

        Ok(())
    }

    #[test]
    fn test_part2() -> Result<(), String> {
        let lines = get_equations();
        assert_eq!(part2(&lines), 11387);

        Ok(())
    }
}
