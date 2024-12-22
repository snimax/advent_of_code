use advent_of_code_2024::{parse_file, parse_lines};

pub fn solve() {
    if let Ok(line_string) = parse_file("Inputs/day22.txt") {
        let lines = parse_lines(&line_string);
        let input = parse_initial_numbers(&lines);
        println!("Part1 solution: {}", part1(&input));
        println!("Part2 solution: {}", part2(&input));
    } else {
        println!("Could not parse file");
    }
}

fn parse_initial_numbers(lines: &[String]) -> Vec<usize> {
    lines.iter().map(|s| s.parse().unwrap()).collect()
}

fn mix_and_prine(current_number: usize, secret_number: usize) -> usize {
    (current_number ^ secret_number) % 16777216
}

fn calculate_next_secret_number(secret_number: usize) -> usize {
    let mul = mix_and_prine(secret_number * 64, secret_number);
    let div = mix_and_prine(mul / 32, mul);
    mix_and_prine(div * 2048, div)
}

fn part1(input: &[usize]) -> usize {
    let mut result = 0;
    for &initial_secret_number in input {
        let mut curr_secret_number = initial_secret_number;
        for _ in 0..2000 {
            curr_secret_number = calculate_next_secret_number(curr_secret_number);
        }
        result += curr_secret_number;
    }
    result
}

fn part2(_input: &[usize]) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_input() -> Vec<usize> {
        let input = r#"1
10
100
2024"#;

        let lines = parse_lines(&input);
        parse_initial_numbers(&lines)
    }

    #[test]
    fn test_part1() -> Result<(), String> {
        let input = get_input();
        assert_eq!(part1(&input), 37327623);

        Ok(())
    }

    #[test]
    fn test_next_secret_number() {
        assert_eq!(calculate_next_secret_number(123), 15887950);
        assert_eq!(calculate_next_secret_number(15887950), 16495136);
        assert_eq!(calculate_next_secret_number(16495136), 527345);
        assert_eq!(calculate_next_secret_number(527345), 704524);
        assert_eq!(calculate_next_secret_number(704524), 1553684);
        assert_eq!(calculate_next_secret_number(1553684), 12683156);
        assert_eq!(calculate_next_secret_number(12683156), 11100544);
        assert_eq!(calculate_next_secret_number(11100544), 12249484);
        assert_eq!(calculate_next_secret_number(12249484), 7753432);
        assert_eq!(calculate_next_secret_number(7753432), 5908254);
    }

    #[test]
    fn test_part2() -> Result<(), String> {
        let input = get_input();
        assert_eq!(part2(&input), 0);

        Ok(())
    }
}
