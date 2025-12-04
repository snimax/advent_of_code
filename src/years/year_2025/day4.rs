use crate::years::AdventDay;

use super::{Map, Pos};

pub struct Day4 {}

impl AdventDay for Day4 {
    fn solve(&self) {
        let lines = self.get_input();
        println!("Part1 solution: {}", part1(&lines));
        println!("Part2 solution: {}", part2(&lines));
    }

    fn get_input_path(&self) -> &str {
        "Inputs/2025/day4.txt"
    }
}

#[derive(Debug, PartialEq)]
enum Space {
    Empty,
    PaperRoll,
}

fn get_occupied_neighbors(map: &Map<Space>, pos: &Pos) -> u32 {
    let mut free_neighbors = 0;
    for y in -1..=1 {
        for x in -1..=1 {
            let curr_pos = Pos {
                x: pos.x + x,
                y: pos.y + y,
            };
            if curr_pos == *pos || !map.valid_pos(&curr_pos) {
                continue;
            }
            if *map.get(&curr_pos) == Space::PaperRoll {
                free_neighbors += 1;
            }
        }
    }
    free_neighbors
}

fn forklift_accessable_papers(map: &Map<Space>) -> Vec<Pos> {
    let mut accessable_paper_positions = Vec::new();
    for y in 0..map.rows() as i32 {
        for x in 0..map.cols() as i32 {
            let curr_pos = Pos { x, y };
            if *map.get(&curr_pos) == Space::PaperRoll && get_occupied_neighbors(map, &curr_pos) < 4
            {
                accessable_paper_positions.push(curr_pos);
            }
        }
    }

    accessable_paper_positions
}

fn parse_map(lines: &[String]) -> Map<Space> {
    Map::new(lines, |c, _pos| match c {
        '.' => Space::Empty,
        '@' => Space::PaperRoll,
        _ => panic!("Found unexpected char '{c}'"),
    })
}

fn part1(lines: &[String]) -> usize {
    let map = parse_map(lines);
    forklift_accessable_papers(&map).len()
}

fn part2(lines: &[String]) -> usize {
    let mut map = parse_map(lines);
    let mut removed_paper_rolls = 0;
    loop {
        let accessable_paper_rolls = forklift_accessable_papers(&map);
        if accessable_paper_rolls.is_empty() {
            break;
        }

        removed_paper_rolls += accessable_paper_rolls.len();
        for removed_paper_roll_pos in accessable_paper_rolls {
            map.set(&removed_paper_roll_pos, Space::Empty);
        }
    }

    removed_paper_rolls
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::library::*;

    fn get_lines() -> Vec<String> {
        let input = r#"..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@."#;

        parse_lines(&input)
    }

    #[test]
    fn test_part1() -> Result<(), String> {
        let lines = get_lines();
        assert_eq!(part1(&lines), 13);

        Ok(())
    }

    #[test]
    fn test_part2() -> Result<(), String> {
        let lines = get_lines();
        assert_eq!(part2(&lines), 43);

        Ok(())
    }
}
