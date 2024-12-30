use advent_of_code_2024::{parse_file, parse_lines, Pos};
use std::collections::HashMap;

pub fn solve() {
    if let Ok(line_string) = parse_file("Inputs/day21.txt") {
        let lines = parse_lines(&line_string);
        println!("Part1 solution: {}", part1(&lines));
        println!("Part2 solution: {}", part2(&lines)); // 248737376587816 is too high
                                                       // 173199917469996 is too low
                                                       // 224189887188196 is too high
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

    frozen_robot_sequence.chars().count()
}

fn get_code_val(code_str: &str) -> usize {
    code_str[..code_str.len() - 1].parse::<usize>().unwrap()
}

fn part1(input: &[String]) -> usize {
    input.iter().fold(0, |acc, line| {
        acc + get_sequence_len(line) * get_code_val(line)
    })
}

fn get_generic_sequence_len(code_str: &str, num_robots: usize) -> usize {
    let mut curr_sequence = get_numpad_sequence(code_str);
    // let mut memoization = HashMap::new();

    let mut result: usize = 0;
    if num_robots <= 4 {
        for _ in 0..num_robots {
            curr_sequence = get_dirpad_sequence(&curr_sequence);//, &mut memoization);
        }
        return curr_sequence.chars().count();
    } else {

        const INITIAL_ROBOT_LAYERS: usize = 3;
        for _ in 0..INITIAL_ROBOT_LAYERS {
            curr_sequence = get_dirpad_sequence(&curr_sequence);//, &mut memoization);
        }

        curr_sequence.insert(0, 'A');

        let mut memoization = HashMap::new();
        for (next, curr) in curr_sequence.chars().skip(1).zip(curr_sequence.chars()) {
            result += get_sequence_len_rec(curr, next, num_robots - INITIAL_ROBOT_LAYERS, &mut memoization)
        }
    }

    result
}

fn get_sequence_len_rec(curr_button: char, target_button: char, depth: usize, memoization: &mut HashMap<(char, char, usize), usize>) -> usize {
    // if its the last keypad, we can press each button immediately
    if depth == 0 {
        return 1;
    }

    if let Some(val) = memoization.get(&(curr_button, target_button, depth)) {
        return *val;
    }

    let curr_button_pos = get_dir_button_coord(curr_button);
    let target_button_pos = get_dir_button_coord(target_button);
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

    // From what I could tell, the longest sequence is 6 chars since I include the initial from A to < => Av<<<A
    let mut curr_sequence_of_moves = String::with_capacity(6);
    curr_sequence_of_moves.push('A'); // We will allways start from A

    if curr_button == '<' {
        curr_sequence_of_moves.push_str(&horizontal_moves);
        curr_sequence_of_moves.push_str(&vertical_moves);
    } else if target_button == '<' {
        curr_sequence_of_moves.push_str(&vertical_moves);
        curr_sequence_of_moves.push_str(&horizontal_moves);
    } else if diff.x <= 0 && diff.y <= 0 { // Up-Left
        curr_sequence_of_moves.push_str(&horizontal_moves);
        curr_sequence_of_moves.push_str(&vertical_moves);
    } else if diff.x <= 0 && diff.y > 0 { // Down-Left
        curr_sequence_of_moves.push_str(&horizontal_moves);
        curr_sequence_of_moves.push_str(&vertical_moves);
    } else if diff.x > 0 && diff.y <= 0 { // Down-right
        curr_sequence_of_moves.push_str(&vertical_moves);
        curr_sequence_of_moves.push_str(&horizontal_moves);
    } else {
        curr_sequence_of_moves.push_str(&vertical_moves);
        curr_sequence_of_moves.push_str(&horizontal_moves);
    }

    curr_sequence_of_moves.push('A');

    let mut required_moves = 0;
    for (next, curr) in curr_sequence_of_moves.chars().skip(1).zip(curr_sequence_of_moves.chars()) {
        required_moves += get_sequence_len_rec(curr, next, depth - 1, memoization);
    }

    memoization.insert((curr_button, target_button, depth), required_moves);

    required_moves
}

fn part2(input: &[String]) -> usize {
    input.iter().fold(0, |acc, line| {
        acc + get_generic_sequence_len(line,25) * get_code_val(line)
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

    fn get_first_real_input() -> Vec<String> {
        let input = r#"480A
682A
140A
246A
938A"#;

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
