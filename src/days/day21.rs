use advent_of_code_2024::{parse_file, parse_lines, Pos};
use std::collections::{HashMap, HashSet, VecDeque};

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

type PossiblePaths = HashMap<(char, char), HashSet<String>>;
type Memoization = HashMap<(char, char, usize), usize>;

fn get_generic_sequence_len(code_str: &str, num_robots: usize) -> usize {
    let dirpad_path_possibilities = build_all_paths();
    let numpad_path_possibilities = build_all_paths_numpad();

    let mut memoization = Memoization::new();
    let mut result = 0;

    let code_str = format!("A{}", code_str); // Need to add that the numpad robot starts at A as well

    for (next, curr) in code_str.chars().skip(1).zip(code_str.chars()) {
        result += get_sequence_len_rec(curr, next, num_robots + 1, &mut memoization, &numpad_path_possibilities, &dirpad_path_possibilities);
    }

    result
}

fn get_sequence_len_rec(curr_button: char, target_button: char, depth: usize, memoization: &mut Memoization, paths_to_use: &PossiblePaths, dir_paths: &PossiblePaths) -> usize {

    if curr_button == target_button || depth == 0 {
        return 1;
    }

    if let Some(path_len) = memoization.get(&(curr_button, target_button, depth)) {
        return *path_len;
    }

    if let Some(possibilities) = paths_to_use.get(&(curr_button, target_button)) {

        let shortest_path = possibilities.iter().map(|possible_path| {
            let path = format!("A{}A", possible_path); // Need to add that we start and end on A each time...
            let mut res = 0;
            for (next, curr) in path.chars().skip(1).zip(path.chars()) {
                res += get_sequence_len_rec(curr, next, depth - 1,  memoization, dir_paths, dir_paths);
            }
            res
        }).min().unwrap();

        memoization.insert((curr_button, target_button, depth), shortest_path);
        return shortest_path;
    }

    usize::MAX
}

fn find_possible_paths(start: char, end: char) -> HashSet<String> {
    let mut paths = HashSet::new();

    let mut queue = VecDeque::new();
    let start_coord = get_dir_button_coord(start);
    let end_coord = get_dir_button_coord(end);
    let diff = end_coord.clone() - start_coord.clone();
    queue.push_back((start_coord, String::new()));

    let (x_move, x_char) = if diff.x < 0 {
        (Pos {x: 1, y:0}, '<')
    } else {
        (Pos {x: -1, y:0}, '>')
    };

    let (y_move, y_char) = if diff.y < 0 {
        (Pos {x: 0, y: 1}, '^')
    } else {
        (Pos {x: 0, y: -1}, 'v')
    };

    let forbidden_pos = Pos{ x:0, y:0 };

    while let Some((curr, path)) = queue.pop_front() {
        if curr == end_coord {
            paths.insert(path.clone());
            continue;
        }

        let horizontal_move_pos = curr.clone() - x_move.clone();
        if horizontal_move_pos != forbidden_pos && horizontal_move_pos.x >= 0 && horizontal_move_pos.x <= 2 {
            let mut new_path = path.clone();
            new_path.push(x_char);
            queue.push_back((horizontal_move_pos, new_path))
        }
        let vertical_move_pos = curr.clone() - y_move.clone();
        if vertical_move_pos != forbidden_pos && vertical_move_pos.y >= 0 && vertical_move_pos.y <= 1 {
            let mut new_path = path.clone();
            new_path.push(y_char);
            queue.push_back((vertical_move_pos, new_path))
        }
    }
    paths
}

fn build_all_paths() -> PossiblePaths {
    const DIR_BUTTONS: [char; 5] = ['^', 'v', '<', '>', 'A'];

    let mut possible_paths = PossiblePaths::new();
    for curr_button in DIR_BUTTONS {
        for target_button in DIR_BUTTONS {
            let paths = find_possible_paths(curr_button, target_button);
            possible_paths.insert((curr_button, target_button), paths);
        }
    }

    possible_paths
}

fn find_possible_paths_numpad(start: char, end: char) -> HashSet<String> {
    let mut paths = HashSet::new();

    let mut queue = VecDeque::new();
    let start_coord = get_numeric_button_coord(start);
    let end_coord = get_numeric_button_coord(end);
    let diff = end_coord.clone() - start_coord.clone();
    queue.push_back((start_coord, String::new()));

    let (x_move, x_char) = if diff.x < 0 {
        (Pos {x: 1, y:0}, '<')
    } else {
        (Pos {x: -1, y:0}, '>')
    };

    let (y_move, y_char) = if diff.y < 0 {
        (Pos {x: 0, y: 1}, '^')
    } else {
        (Pos {x: 0, y: -1}, 'v')
    };

    let forbidden_pos = Pos{ x:0, y:3 };

    while let Some((curr, path)) = queue.pop_front() {
        if curr == end_coord {
            paths.insert(path.clone());
            continue;
        }

        let horizontal_move_pos = curr.clone() - x_move.clone();
        if horizontal_move_pos != forbidden_pos && horizontal_move_pos.x >= 0 && horizontal_move_pos.x <= 2 {
            let mut new_path = path.clone();
            new_path.push(x_char);
            queue.push_back((horizontal_move_pos, new_path))
        }
        let vertical_move_pos = curr.clone() - y_move.clone();
        if vertical_move_pos != forbidden_pos && vertical_move_pos.y >= 0 && vertical_move_pos.y <= 3 {
            let mut new_path = path.clone();
            new_path.push(y_char);
            queue.push_back((vertical_move_pos, new_path))
        }
    }
    paths
}

fn build_all_paths_numpad() -> PossiblePaths {
    const DIR_BUTTONS: [char; 11] = ['1','2','3','4','5','6','7','8','9','0','A'];

    let mut possible_paths = PossiblePaths::new();
    for curr_button in DIR_BUTTONS {
        for target_button in DIR_BUTTONS {
            let paths = find_possible_paths_numpad(curr_button, target_button);
            possible_paths.insert((curr_button, target_button), paths);
        }
    }

    possible_paths
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
        assert_eq!(part2(&input), 154115708116294);

        Ok(())
    }
}
