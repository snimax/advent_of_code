use super::{DOWN, Dir, LEFT, Map, Pos, RIGHT, UP};
use std::collections::HashSet;

use crate::years::AdventDay;

pub struct Day6 {}

impl AdventDay for Day6 {
    fn solve(&self) {
        let lines = self.get_input();
        let (mut map, start_pos) = parse_map(&lines);
        println!("Part1 solution: {}", part1(&map, &start_pos));
        println!("Part2 solution: {}", part2(&mut map, &start_pos));
    }

    fn get_input_path(&self) -> &str {
        "Inputs/2024/day6.txt"
    }
}

fn turn_right(dir: &mut &Dir) {
    *dir = match *dir {
        UP => RIGHT,
        RIGHT => DOWN,
        DOWN => LEFT,
        LEFT => UP,
        _ => panic!("Direction not possible {:?}", dir),
    }
}

fn parse_map(lines: &[String]) -> (Map<u8>, Pos) {
    let mut start_pos = Pos { x: 0, y: 0 };
    let map = Map::new(lines, |char, pos| {
        if char == '^' {
            start_pos = pos.to_owned();
            return b'.';
        }
        match char {
            '#' => b'#',
            '.' => b'.',
            _ => panic!("Got unexpected char '{char}' while parsing map"),
        }
    });
    (map, start_pos)
}

fn find_visited_positions(map: &Map<u8>, start_pos: &Pos, dir: &Dir) -> Option<HashSet<Pos>> {
    let mut curr_pos = start_pos.clone();
    let mut dir = dir;

    let mut visited_positions = HashSet::new();
    visited_positions.insert(curr_pos.clone());

    let mut path = HashSet::new();
    path.insert((curr_pos.clone(), dir.clone()));

    while let Some(val) = map.next(&curr_pos, dir) {
        match val {
            b'.' => {
                curr_pos += dir;
                visited_positions.insert(curr_pos.clone());
                if path.contains(&(curr_pos.clone(), dir.clone())) {
                    return None;
                }
                path.insert((curr_pos.clone(), dir.clone()));
            }
            b'#' => turn_right(&mut dir),
            _ => panic!("got unexpected value from map {}", val),
        }
    }
    Some(visited_positions)
}

fn part1(map: &Map<u8>, start_pos: &Pos) -> usize {
    let visited_positions = find_visited_positions(map, start_pos, UP);
    visited_positions.unwrap().len()
}

fn part2(map: &mut Map<u8>, start_pos: &Pos) -> usize {
    let mut curr_pos = start_pos.clone();
    let mut dir = UP;

    let mut visited_positions = HashSet::new();
    visited_positions.insert(curr_pos.clone());

    let mut result = 0;

    while let Some(val) = map.next(&curr_pos, dir) {
        match val {
            b'.' => {
                let next_pos = &curr_pos + dir;
                // No use in putting out obsticles where one already exists
                if *map.get(&next_pos) == b'#' {
                    continue;
                }

                if !visited_positions.contains(&next_pos) {
                    map.set(&next_pos, b'#');
                    if find_visited_positions(map, &curr_pos, dir).is_none() {
                        result += 1
                    }
                    map.set(&next_pos, b'.');
                }

                curr_pos = next_pos;
                visited_positions.insert(curr_pos.clone());
            }
            b'#' => turn_right(&mut dir),
            _ => panic!("got unexpected value from map {}", val),
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::library::*;

    fn get_input() -> (Map<u8>, Pos) {
        let input = r#"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#..."#;

        parse_map(&parse_lines(&input))
    }

    #[test]
    fn test_part1() -> Result<(), String> {
        let (map, start_pos) = get_input();
        assert_eq!(part1(&map, &start_pos), 41);

        Ok(())
    }

    #[test]
    fn test_part2() -> Result<(), String> {
        let (mut map, start_pos) = get_input();
        assert_eq!(part2(&mut map, &start_pos), 6);

        Ok(())
    }
}
