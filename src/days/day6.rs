use advent_of_code_2024::{parse_file, parse_lines};
use std::collections::HashSet;
use std::ops::Add;

pub fn solve() {
    if let Ok(line_string) = parse_file("Inputs/day6.txt") {
        let lines = parse_lines(&line_string);
        let (map, start_pos) = parse_map(&lines);
        println!("Part1 solution: {}", part1(&map, &start_pos));
        println!("Part2 solution: {}", part2(&map, &start_pos));
    } else {
        println!("Could not parse file");
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Pos {
    x: i32,
    y: i32,
}

impl Add for Pos {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

type Dir = Pos;

const UP: Dir = Dir { x: 0, y: -1 };
const DOWN: Dir = Dir { x: 0, y: 1 };
const LEFT: Dir = Dir { x: -1, y: 0 };
const RIGHT: Dir = Dir { x: 1, y: 0 };

fn get_next_dir(dir: &Dir) -> Dir {
    match *dir {
        UP => RIGHT,
        RIGHT => DOWN,
        DOWN => LEFT,
        LEFT => UP,
        _ => panic!("Direction not possible {:?}", dir),
    }
}

#[derive(Clone)]
struct Map {
    map: Vec<Vec<u8>>,
    size_x: usize,
    size_y: usize,
}

impl Map {
    fn next(&self, curr_pos: &Pos, dir: &Dir) -> Option<u8> {
        let new_pos = Pos {
            x: curr_pos.x + dir.x,
            y: curr_pos.y + dir.y,
        };
        if new_pos.x < 0
            || new_pos.y < 0
            || new_pos.x >= self.size_x as i32
            || new_pos.y >= self.size_y as i32
        {
            return None;
        }

        Some(self.get(&new_pos))
    }

    fn get(&self, pos: &Pos) -> u8 {
        self.map[pos.y as usize][pos.x as usize]
    }
}

fn parse_map(lines: &[String]) -> (Map, Pos) {
    let size_y = lines.len();
    let size_x = lines[0].len();

    let mut map = vec![vec![]; size_y];
    let mut start_pos = Pos { x: 0, y: 0 };

    map.iter_mut()
        .zip(lines)
        .for_each(|(map_row, line)| *map_row = line.as_bytes().to_vec());

    lines.iter().enumerate().for_each(|(row, line)| {
        line.as_bytes().iter().enumerate().for_each(|(col, val)| {
            if *val == b'^' {
                start_pos = Pos {
                    x: col as i32,
                    y: row as i32,
                };
            }
        })
    });

    map[start_pos.y as usize][start_pos.x as usize] = b'.';

    (
        Map {
            map,
            size_x,
            size_y,
        },
        start_pos,
    )
}

fn found_loop(path: &Vec<Pos>) -> bool {
    if path.len() < 4 {
        return false;
    }
    let last = path[path.len() - 1].clone();
    let second_last = path[path.len() - 2].clone();

    for i in 0..path.len()-3
    {
        if path[i] == second_last && path[i + 1] == last {
            return true;
        }
    }

    false
}

fn find_visited_positions(map: &Map, start_pos: &Pos) -> Option<HashSet<Pos>> {
    let mut curr_pos = start_pos.clone();
    let mut dir = UP;

    let mut visited_positions = HashSet::new();
    visited_positions.insert(curr_pos.clone());

    let mut path = vec![curr_pos.clone()];

    while let Some(val) = map.next(&curr_pos, &dir) {
        if found_loop(&path) {
            return None;
        }
        match val {
            b'.' => {
                curr_pos = curr_pos + dir.clone();
                visited_positions.insert(curr_pos.clone());
                path.push(curr_pos.clone())
            }
            b'#' => dir = get_next_dir(&dir),
            _ => panic!("got unexpected value from map {}", val),
        }
    }
    Some(visited_positions)
}

fn part1(map: &Map, start_pos: &Pos) -> usize {
    let visited_positions = find_visited_positions(&map, &start_pos);
    visited_positions.unwrap().len()
}

fn part2(map: &Map, start_pos: &Pos) -> usize {
    let visited_positions = find_visited_positions(&map, &start_pos).unwrap();

    let mut result = 0;
    for (idx, pos) in visited_positions.iter().filter(|&pos| pos != start_pos).enumerate() {
        let mut new_map = map.clone();
        new_map.map[pos.y as usize][pos.x as usize] = b'#';
        println!("Testing solution {idx}");
        if find_visited_positions(&new_map, start_pos) == None {
            result += 1;
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_input() -> (Map, Pos) {
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
        let (map, start_pos) = get_input();
        assert_eq!(part2(&map, &start_pos), 6);

        Ok(())
    }
}
