use super::{parse_file, parse_lines};
use std::collections::HashMap;

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

const ITERATIONS: usize = 2000;

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
        for _ in 0..ITERATIONS {
            curr_secret_number = calculate_next_secret_number(curr_secret_number);
        }
        result += curr_secret_number;
    }
    result
}

fn generate_diff_sequence(initial_secret_number: usize) -> (Vec<usize>, Vec<i32>) {
    let mut least_significant_digits = vec![initial_secret_number % 10; ITERATIONS + 1];
    let mut curr_secret_number = initial_secret_number;
    for index in 0..ITERATIONS {
        curr_secret_number = calculate_next_secret_number(curr_secret_number);
        least_significant_digits[index + 1] = curr_secret_number % 10;
    }

    let mut differences = vec![0; ITERATIONS];
    for (idx, (a, b)) in least_significant_digits
        .iter()
        .skip(1)
        .zip(least_significant_digits.iter())
        .enumerate()
    {
        differences[idx] = *a as i32 - *b as i32;
    }

    least_significant_digits.remove(0);
    (least_significant_digits, differences)
}

fn get_sequences(prices: &[usize], differences: &[i32]) -> HashMap<[i32; 4], usize> {
    let mut sequence_values = HashMap::new();

    for index in 3..differences.len() {
        let sequence = [
            differences[index - 3],
            differences[index - 2],
            differences[index - 1],
            differences[index],
        ];

        if sequence_values.contains_key(&sequence) {
            continue;
        }
        sequence_values.insert(sequence, prices[index]);
    }

    sequence_values
}

fn part2(input: &[usize]) -> usize {
    let mut sequence_values = HashMap::new();
    for &secret_num in input {
        let (prices, differences) = generate_diff_sequence(secret_num);

        let values = get_sequences(&prices, &differences);
        for (k, v) in values {
            if let Some(val) = sequence_values.get_mut(&k) {
                *val += v;
            } else {
                sequence_values.insert(k, v);
            }
        }
    }
    if let Some((_, max_val)) = sequence_values.iter().max_by_key(|(_, v)| *v) {
        *max_val
    } else {
        0
    }
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

    fn get_input2() -> Vec<usize> {
        let input = r#"1
2
3
2024"#;

        let lines = parse_lines(&input);
        parse_initial_numbers(&lines)
    }

    #[test]
    fn test_part2() -> Result<(), String> {
        let input = get_input2();
        assert_eq!(part2(&input), 23);

        Ok(())
    }
}
