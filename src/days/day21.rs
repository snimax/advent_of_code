use advent_of_code_2024::{parse_file, parse_lines, Pos};
use std::{collections::HashMap, ffi::IntoStringError};

pub fn solve() {
    if let Ok(line_string) = parse_file("Inputs/day21.txt") {
        let lines = parse_lines(&line_string);
        println!("Part1 solution: {}", part1(&lines));
        println!("Part2 solution: {}", part2(&lines));
    } else {
        println!("Could not parse file");
    }
}

fn get_numeric_button_coord(c: char) -> Pos {
    match c {
        '0' => Pos { x: 1, y: 3 },
        '1' => Pos { x: 0, y: 2 },
        '2' => Pos { x: 1, y: 2 },
        '3' => Pos { x: 2, y: 2 },
        '4' => Pos { x: 0, y: 1 },
        '5' => Pos { x: 1, y: 1 },
        '6' => Pos { x: 2, y: 1 },
        '7' => Pos { x: 0, y: 0 },
        '8' => Pos { x: 1, y: 0 },
        '9' => Pos { x: 2, y: 0 },
        'A' => Pos { x: 2, y: 3 },
        c => panic!("Got unrecognized button '{c}'!"),
    }
}

fn get_dir_button_coord(c: char) -> Pos {
    match c {
        '^' => Pos { x: 1, y: 0 },
        '<' => Pos { x: 0, y: 1 },
        'v' => Pos { x: 1, y: 1 },
        '>' => Pos { x: 2, y: 1 },
        'A' => Pos { x: 2, y: 0 },
        c => panic!("Got unrecognized button '{c}'!"),
    }
}

fn get_numpad_sequence(code_str: &str) -> String {
    let mut curr_button_pos = get_numeric_button_coord('A');
    let mut instruction_sequence = String::new();

    for c in code_str.chars() {
        let target_button_pos = get_numeric_button_coord(c);
        let diff = target_button_pos.clone() - curr_button_pos.clone();

        let horizontal_moves = match diff.x.cmp(&0) {
            std::cmp::Ordering::Less => "<",
            std::cmp::Ordering::Equal => "",
            std::cmp::Ordering::Greater => ">",
        }
        .repeat(diff.x.unsigned_abs() as usize);

        let vertical_moves = match diff.y.cmp(&0) {
            std::cmp::Ordering::Less => "^",
            std::cmp::Ordering::Equal => "",
            std::cmp::Ordering::Greater => "v",
        }
        .repeat(diff.y.unsigned_abs() as usize);

        if curr_button_pos.x == 1 && target_button_pos.y == 3 && target_button_pos.y != 0 {
            instruction_sequence.push_str(&vertical_moves);
            instruction_sequence.push_str(&horizontal_moves);
        } else if !(curr_button_pos.y == 3 && target_button_pos.x == 0) {
            instruction_sequence.push_str(&horizontal_moves);
            instruction_sequence.push_str(&vertical_moves);
        } else if !(curr_button_pos.x == 0 && target_button_pos.y == 3) {
            instruction_sequence.push_str(&vertical_moves);
            instruction_sequence.push_str(&horizontal_moves);
        }

        instruction_sequence.push('A');

        curr_button_pos = target_button_pos;
    }

    instruction_sequence
}

fn get_dirpad_sequence(sequence_str: &str) -> String {
    let mut curr_button_pos = get_dir_button_coord('A');
    let mut instruction_sequence = String::new();

    for c in sequence_str.chars() {
        let target_button_pos = get_dir_button_coord(c);
        let diff = target_button_pos.clone() - curr_button_pos.clone();

        let horizontal_moves = match diff.x.cmp(&0) {
            std::cmp::Ordering::Less => "<",
            std::cmp::Ordering::Equal => "",
            std::cmp::Ordering::Greater => ">",
        }
        .repeat(diff.x.unsigned_abs() as usize);

        let vertical_moves = match diff.y.cmp(&0) {
            std::cmp::Ordering::Less => "^",
            std::cmp::Ordering::Equal => "",
            std::cmp::Ordering::Greater => "v",
        }
        .repeat(diff.y.unsigned_abs() as usize);

        if !(curr_button_pos.y == 0 && target_button_pos.x == 0) {
            instruction_sequence.push_str(&horizontal_moves);
            instruction_sequence.push_str(&vertical_moves);
        } else if !(curr_button_pos.x == 0 && target_button_pos.y == 0) {
            instruction_sequence.push_str(&vertical_moves);
            instruction_sequence.push_str(&horizontal_moves);
        }

        instruction_sequence.push('A');
        curr_button_pos = target_button_pos;
    }
    instruction_sequence
}

fn get_sequence_len(code_str: &str) -> usize {
    let numpad_robot_sequence = get_numpad_sequence(code_str);
    let radioactive_robot_sequence = get_dirpad_sequence(&numpad_robot_sequence);
    let frozen_robot_sequence = get_dirpad_sequence(&radioactive_robot_sequence);

    frozen_robot_sequence
        .chars()
        .count()
}

fn get_code_val(code_str: &str) -> usize {
    code_str[..code_str.len() - 1].parse::<usize>().unwrap()
}

fn part1(input: &[String]) -> usize {
    input.iter().fold(0, |acc, line| {
        acc + get_sequence_len(line) * get_code_val(line)
    })
}

fn get_dirpad_sequence1(sequence_str: &str, memoization: &mut HashMap<(Pos, Pos), String>) -> String {
    let mut curr_button_pos = get_dir_button_coord('A');
    let mut instruction_sequence = String::with_capacity(sequence_str.len() * 4);

    for c in sequence_str.chars() {
        let target_button_pos = get_dir_button_coord(c);
        let diff = target_button_pos.clone() - curr_button_pos.clone();

        if let Some(str) = memoization.get(&(curr_button_pos.clone(), target_button_pos.clone())) {
            instruction_sequence.push_str(str);
            continue;
        }

        let horizontal_moves = match diff.x.cmp(&0) {
            std::cmp::Ordering::Less => "<",
            std::cmp::Ordering::Equal => "",
            std::cmp::Ordering::Greater => ">",
        }
        .repeat(diff.x.unsigned_abs() as usize);

        let vertical_moves = match diff.y.cmp(&0) {
            std::cmp::Ordering::Less => "^",
            std::cmp::Ordering::Equal => "",
            std::cmp::Ordering::Greater => "v",
        }
        .repeat(diff.y.unsigned_abs() as usize);

        let mut curr_sequence_of_moves = String::with_capacity(4);

        if !(curr_button_pos.y == 0 && target_button_pos.x == 0) {
            curr_sequence_of_moves.push_str(&horizontal_moves);
            curr_sequence_of_moves.push_str(&vertical_moves);
        } else if !(curr_button_pos.x == 0 && target_button_pos.y == 0) {
            curr_sequence_of_moves.push_str(&vertical_moves);
            curr_sequence_of_moves.push_str(&horizontal_moves);
        }

        curr_sequence_of_moves.push('A');

        memoization.insert((curr_button_pos.clone(), target_button_pos.clone()), curr_sequence_of_moves.clone());
        instruction_sequence.push_str(&curr_sequence_of_moves);

        curr_button_pos = target_button_pos;
    }
    instruction_sequence
}

fn get_generic_sequence_len(code_str: &str, num_robots: usize) -> usize {
    let mut curr_sequence = get_numpad_sequence(code_str);
    let mut memoization = HashMap::new();

    for i in 0..num_robots {
        println!("{i}: {}", curr_sequence.len());
        curr_sequence = get_dirpad_sequence1(&curr_sequence, &mut memoization);
    }

    curr_sequence
        .chars()
        .count()
}

fn part2(input: &[String]) -> usize {
    input.iter().fold(0, |acc, line| {
        acc + get_generic_sequence_len(line, 25) * get_code_val(line)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_input() -> Vec<String> {
        let input = r#"029A
980A
179A
456A
379A"#;

        parse_lines(&input)
    }

    #[test]
    fn test_part1() -> Result<(), String> {
        let input = get_input();
        assert_eq!(part1(&input), 126384);

        Ok(())
    }

    #[test]
    fn test_get_numpad_dir() {
        assert_eq!(get_numpad_sequence("029A").len(), 12);
    }

    #[test]
    fn test_get_first_dirpad_dir() {
        let numpad_sequence = get_numpad_sequence("029A");
        assert_eq!(get_dirpad_sequence(&numpad_sequence).len(), 28);
    }

    #[test]
    fn test_get_second_dirpad_dir() {
        let numpad_sequence = get_numpad_sequence("029A");
        let dirpad_sequence = get_dirpad_sequence(&numpad_sequence);
        assert_eq!(get_dirpad_sequence(&dirpad_sequence).len(), 68);
    }

    #[test]
    fn test_get_second_dirpad_dir2() {
        let numpad_sequence = get_numpad_sequence("62A");
        let dirpad_sequence = get_dirpad_sequence(&numpad_sequence);
        assert_eq!(
            get_dirpad_sequence(&dirpad_sequence).len(),
            "v<<A>>^AAvA^A<vA<AA>>^AvA^A<Av>A^Av<<A>A^>AvA^A<A>A".len()
        );
    }

    #[test]
    fn test_get_second_dirpad_dir3() {
        let numpad_sequence = get_numpad_sequence("26A");
        let dirpad_sequence = get_dirpad_sequence(&numpad_sequence);
        assert_eq!(get_dirpad_sequence(&dirpad_sequence).len(), 57);
    }

    #[test]
    fn test_part2() -> Result<(), String> {
        let input = get_input();
        assert_eq!(part2(&input), 126384);

        Ok(())
    }
}
