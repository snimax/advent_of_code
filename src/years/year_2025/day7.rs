use crate::years::AdventDay;

use std::collections::HashMap;

pub struct Day7 {}

impl AdventDay for Day7 {
    fn solve(&self) {
        let lines = self.get_input();
        println!("Part1 solution: {}", part1(&lines));
        println!("Part2 solution: {}", part2(&lines));
    }

    fn get_input_path(&self) -> &str {
        "Inputs/2025/day7.txt"
    }
}

fn find_start_pos(lines: &[String]) -> Option<usize> {
    lines.first().and_then(|s| s.find('S'))
}

fn beam_should_split(line: &str, pos: usize) -> bool {
    line.chars().nth(pos).is_some_and(|c| c == '^')
}

fn find_split_beams(lines: &[String]) -> usize {
    let mut start_beam_positions = if let Some(pos) = find_start_pos(lines) {
        vec![pos]
    } else {
        return 0;
    };

    let mut beam_splits = 0;
    for line in lines {
        let mut new_beam_positions = Vec::new();
        for beam_pos in start_beam_positions.iter() {
            if beam_should_split(line, *beam_pos) {
                new_beam_positions.push(beam_pos - 1);
                new_beam_positions.push(beam_pos + 1);
                beam_splits += 1;
            } else {
                // Beam continues downward
                new_beam_positions.push(*beam_pos);
            }
        }
        new_beam_positions.sort();
        new_beam_positions.dedup();
        start_beam_positions = new_beam_positions;
    }
    beam_splits
}

fn part1(lines: &[String]) -> usize {
    find_split_beams(lines)
}

fn find_timeline_splits(
    lines: &[String],
    curr_depth: usize,
    curr_pos: usize,
    memoization: &mut HashMap<(usize, usize), usize>,
) -> usize {
    let key = (curr_depth, curr_pos);
    if let Some(res) = memoization.get(&key) {
        return *res;
    }

    if curr_depth >= lines.len() {
        return 1;
    }
    let new_depth = curr_depth + 1;

    let value = if beam_should_split(&lines[curr_depth], curr_pos) {
        find_timeline_splits(lines, new_depth, curr_pos - 1, memoization)
            + find_timeline_splits(lines, new_depth, curr_pos + 1, memoization)
    } else {
        find_timeline_splits(lines, new_depth, curr_pos, memoization)
    };

    memoization.insert(key, value);
    value
}

fn part2(lines: &[String]) -> usize {
    let start_beam_position = if let Some(pos) = find_start_pos(lines) {
        pos
    } else {
        return 0;
    };
    let mut memoization = HashMap::new();
    find_timeline_splits(lines, 0, start_beam_position, &mut memoization)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::library::*;

    fn get_lines() -> Vec<String> {
        let input = r#".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
..............."#;

        parse_lines(&input)
    }

    #[test]
    fn test_part1() -> Result<(), String> {
        let lines = get_lines();
        assert_eq!(part1(&lines), 21);

        Ok(())
    }

    #[test]
    fn test_part2() -> Result<(), String> {
        let lines = get_lines();
        assert_eq!(part2(&lines), 40);

        Ok(())
    }
}
