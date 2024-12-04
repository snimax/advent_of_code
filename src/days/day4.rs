use advent_of_code_2024::{parse_file, parse_lines};

pub fn solve() {
    if let Ok(line_string) = parse_file("Inputs/day4.txt") {
        let lines = parse_lines(&line_string);
        println!("Part1 solution: {}", part1(&lines));
        println!("Part2 solution: {}", part2(&lines));
    } else {
        println!("Could not parse file");
    }
}

fn build_transposed_list(lines: &[String]) -> Vec<String> {
    let mut list: Vec<String> = vec![String::new(); lines.len()];

    for line in lines {
        for (idx, char) in line.chars().enumerate() {
            list[idx].push(char);
        }
    }

    list
}

fn build_diagonal_list(lines: &[String], reverse: bool) -> Vec<String> {
    let mut list = vec![String::new(); lines.len()];

    let len = lines.len();
    if reverse {
        for (idx, line) in lines.iter().enumerate() {
            list[idx] = ".".repeat(idx);
            list[idx].insert_str(idx, line);
        }
    } else {
        for (idx, line) in lines.iter().enumerate() {
            let nr: i32 = (len as i32 - idx as i32) - 1;
            let str = ".".repeat(nr as usize);
            list[idx] = str;
            list[idx].insert_str(lines.len() - idx - 1, line);
        }
    }
    list
}

fn find_match(list: &[String], row: usize, col: usize) -> usize {
    if list.len() < (row + 4) {
        return 0;
    }
    if list[row].len() < col || list[row + 3].len() < col {
        return 0;
    }

    let row0_char = list[row].chars().nth(col);
    let row1_char = list[row + 1].chars().nth(col);
    let row2_char = list[row + 2].chars().nth(col);
    let row3_char = list[row + 3].chars().nth(col);

    if row0_char == Some('X')
        && row1_char == Some('M')
        && row2_char == Some('A')
        && row3_char == Some('S')
    {
        return 1;
    } else if row0_char == Some('S')
        && row1_char == Some('A')
        && row2_char == Some('M')
        && row3_char == Some('X')
    {
        return 1;
    }

    0
}

fn find_matches(list: &[String]) -> usize {
    let mut matches = 0;
    for row in 0..list.len() {
        for col in 0..list[row].len() {
            matches += find_match(list, row, col);
        }
    }
    matches
}

fn part1(lines: &[String]) -> usize {
    let transposed_lines = build_transposed_list(lines);

    let mut result = 0;

    result += find_matches(lines);
    result += find_matches(&transposed_lines);
    let diagonal_lines = build_diagonal_list(lines, true);
    result += find_matches(&diagonal_lines);

    let diagonal_lines = build_diagonal_list(lines, false);
    result += find_matches(&diagonal_lines);
    result
}

fn find_match_part2(list: &[String], row: usize, col: usize) -> usize {
    if list.len() < (row + 3) {
        return 0;
    }
    if list[row].len() < (col + 3) {
        return 0;
    }

    let mut potential_match = false;
    if let Some('A') = list[row + 1].chars().nth(col + 1) {
        if let Some('M') = list[row].chars().nth(col) {
            if let Some('S') = list[row + 2].chars().nth(col + 2) {
                potential_match = true;
            }
        } else if let Some('S') = list[row].chars().nth(col) {
            if let Some('M') = list[row + 2].chars().nth(col + 2) {
                potential_match = true;
            }
        }

        if potential_match {
            if let Some('M') = list[row + 2].chars().nth(col) {
                if let Some('S') = list[row].chars().nth(col + 2) {
                    return 1;
                }
            } else if let Some('S') = list[row + 2].chars().nth(col) {
                if let Some('M') = list[row].chars().nth(col + 2) {
                    return 1;
                }
            }
        }
    }

    0
}

fn part2(lines: &[String]) -> usize {
    let mut matches = 0;
    for row in 0..lines.len() {
        for col in 0..lines[row].len() {
            matches += find_match_part2(lines, row, col);
        }
    }
    matches
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_lines() -> Vec<String> {
        let input = r#"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"#;

        parse_lines(&input)
    }

    #[test]
    fn test_part1() -> Result<(), String> {
        let lines = get_lines();
        assert_eq!(part1(&lines), 18);

        Ok(())
    }

    #[test]
    fn test_part2() -> Result<(), String> {
        let lines = get_lines();
        assert_eq!(part2(&lines), 9);

        Ok(())
    }
}
