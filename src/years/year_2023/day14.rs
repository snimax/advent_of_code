use std::{collections::HashMap, ops::Rem};

use super::{DOWN, Dir, LEFT, Map, Pos, RIGHT, UP};
use crate::years::AdventDay;

pub struct Day14 {}

impl AdventDay for Day14 {
    fn solve(&self) {
        let lines = self.get_input();
        let (map, rocks) = parse_map(&lines);
        println!("Part1 solution: {}", part1(&map, &rocks));
        println!("Part2 solution: {}", part2(&map, &rocks));
    }

    fn get_input_path(&self) -> &str {
        "Inputs/2023/day14.txt"
    }
}

#[derive(Debug, Clone, PartialEq)]
enum Space {
    Empty,
    Pillar,
}
type Rock = Pos;

fn parse_map(lines: &[String]) -> (Map<Space>, Vec<Rock>) {
    let mut rocks = Vec::new();
    let map = Map::new(lines, |char, pos| match char {
        '.' => Space::Empty,
        '#' => Space::Pillar,
        'O' => {
            rocks.push(pos.to_owned());
            Space::Empty
        }
        _ => panic!(
            "Got unexpected char '{char}' when parsing map. Only expect chars '.', 'O' and '#' in a grid."
        ),
    });
    (map, rocks)
}

fn sort_rocks(rocks: &mut [Rock]) {
    rocks.sort_by(|a, b| match a.y.cmp(&b.y) {
        std::cmp::Ordering::Equal => a.x.cmp(&b.y),
        cmp => cmp,
    });
}

fn calculate_load(rocks: &[Rock], map: &Map<Space>) -> usize {
    rocks
        .iter()
        .map(|rock| map.rows() - (rock.y) as usize)
        .sum()
}

fn move_rocks(dir: &Dir, rocks: &mut Vec<Rock>, map: &Map<Space>) {
    sort_rocks(rocks);

    let mut new_rock_positions: Vec<Rock> = Vec::new();
    let mut rock_moved = false;

    for rock in rocks.iter() {
        let new_rock_pos = rock + dir;
        if map.valid_pos(&new_rock_pos)
            && *map.get(&new_rock_pos) != Space::Pillar
            && !rocks.contains(&new_rock_pos)
        {
            new_rock_positions.push(new_rock_pos);
            rock_moved = true;
        } else {
            new_rock_positions.push(rock.to_owned())
        }
    }

    if rock_moved {
        *rocks = new_rock_positions;
        move_rocks(dir, rocks, map)
    }
}

fn part1(map: &Map<Space>, initial_rocks: &[Rock]) -> usize {
    let mut rocks = initial_rocks.to_owned();
    move_rocks(UP, &mut rocks, map);

    calculate_load(&rocks, map)
}

fn part2(map: &Map<Space>, initial_rocks: &[Rock]) -> usize {
    let mut seen_states = HashMap::new();
    seen_states.insert(initial_rocks.to_owned(), 0);

    let mut curr_rock_positions = initial_rocks.to_owned();
    const DIRECTIONS: [&Dir; 4] = [UP, LEFT, DOWN, RIGHT];
    let states_before_cycle = loop {
        for dir in DIRECTIONS.iter() {
            move_rocks(dir, &mut curr_rock_positions, map);
        }
        println!("{}", seen_states.len());

        if let Some(val) = seen_states.get(&curr_rock_positions) {
            break val;
        }
        seen_states.insert(curr_rock_positions.clone(), seen_states.len());
    };

    let cycle = seen_states.len() - states_before_cycle;
    let midx = (1000000000 - states_before_cycle).rem(cycle) + states_before_cycle;

    println!("cycle: {cycle}, states_before_cycle: {states_before_cycle}");
    let final_rock_positions = seen_states
        .iter()
        .find(|(_, v)| **v == midx)
        .map(|(k, _)| k)
        .unwrap();

    calculate_load(final_rock_positions, map)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::library::*;

    #[test]
    fn test_part1() -> Result<(), String> {
        let (map, rocks) = get_input();
        assert_eq!(part1(&map, &rocks), 136);

        Ok(())
    }

    #[test]
    fn test_part2() -> Result<(), String> {
        let (map, rocks) = get_input();
        assert_eq!(part2(&map, &rocks), 64);

        Ok(())
    }

    fn get_input<'a>() -> (Map<Space>, Vec<Rock>) {
        let input = r#"O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#...."#;

        parse_map(&parse_lines(&input))
    }
}
