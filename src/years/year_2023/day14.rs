use std::{
    collections::HashMap,
    ops::{Range, Rem},
};

use super::{DOWN, Dir, LEFT, Pos, RIGHT, UP};
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

type Rock = Pos;
type FreeRanges = Vec<Range<i32>>;

struct Map {
    horizontal_ranges: Vec<FreeRanges>,
    vertical_ranges: Vec<FreeRanges>,
    cols: i32,
    rows: i32,
}

fn parse_map(lines: &[String]) -> (Map, Vec<Rock>) {
    let mut rocks = Vec::new();
    let mut pillars = Vec::new();

    let rows = lines.len() as i32;
    let cols = lines[0].len() as i32;

    for (row, line) in lines.iter().enumerate() {
        for (col, c) in line.chars().enumerate() {
            let pos = Pos {
                x: col as i32,
                y: row as i32,
            };
            match c {
                'O' => rocks.push(pos),
                '#' => pillars.push(pos),
                _ => {}
            }
        }
    }

    let mut vertical_ranges: Vec<FreeRanges> = Vec::with_capacity(rows as usize);
    for col in 0..(cols) {
        let mut pillars_in_col = pillars
            .iter()
            .filter(|pillar| pillar.x == col)
            .map(|pillar: &Pos| pillar.y)
            .collect::<Vec<_>>();
        // Handle edge cases
        pillars_in_col.push(-1);
        pillars_in_col.push(rows);
        pillars_in_col.sort();

        let mut vertical_ranges_in_col = Vec::with_capacity(pillars_in_col.len() - 1);

        for (pillar, next_pillar) in pillars_in_col.iter().zip(pillars_in_col.iter().skip(1)) {
            let pillar_range = *pillar + 1..*next_pillar;
            if !pillar_range.is_empty() {
                vertical_ranges_in_col.push(pillar_range);
            }
        }
        vertical_ranges.push(vertical_ranges_in_col);
    }

    let mut horizontal_ranges: Vec<FreeRanges> = Vec::with_capacity(cols as usize);
    for row in 0..(rows) {
        let mut pillars_in_row = pillars
            .iter()
            .filter(|pillar| pillar.y == row)
            .map(|pillar: &Pos| pillar.x)
            .collect::<Vec<_>>();
        // Handle edge cases
        pillars_in_row.push(-1);
        pillars_in_row.push(cols);
        pillars_in_row.sort();

        let mut horizontal_ranges_in_row = Vec::with_capacity(pillars_in_row.len() - 1);

        for (pillar, next_pillar) in pillars_in_row.iter().zip(pillars_in_row.iter().skip(1)) {
            let pillar_range = *pillar + 1..*next_pillar;
            if !pillar_range.is_empty() {
                horizontal_ranges_in_row.push(pillar_range);
            }
        }
        horizontal_ranges.push(horizontal_ranges_in_row);
    }

    let map = Map {
        horizontal_ranges,
        vertical_ranges,
        cols,
        rows,
    };

    (map, rocks)
}

fn calculate_load(rocks: &[Rock], map: &Map) -> usize {
    rocks
        .iter()
        .map(|rock| map.rows as usize - (rock.y) as usize)
        .sum()
}

fn part1(map: &Map, initial_rocks: &[Rock]) -> usize {
    let mut rocks = initial_rocks.to_owned();
    move_rocks_smarter(UP, &mut rocks, map);

    calculate_load(&rocks, map)
}

fn move_rocks_smarter(dir: &Dir, rocks: &mut [Rock], map: &Map) {
    match dir {
        UP => move_rocks_up(rocks, map),
        DOWN => move_rocks_down(rocks, map),
        LEFT => move_rocks_left(rocks, map),
        RIGHT => move_rocks_right(rocks, map),
        _ => panic!("Got unknown direction {dir:?}"),
    }
}

fn move_rocks_up(rocks: &mut [Rock], map: &Map) {
    rocks.sort_by(|a, b| match a.y.cmp(&b.y) {
        std::cmp::Ordering::Equal => a.x.cmp(&b.x),
        cmp => cmp,
    });

    for col in 0..map.cols {
        for pillar_range in map.vertical_ranges[col as usize].iter() {
            rocks
                .iter_mut()
                .filter(|rock| rock.x == col && pillar_range.contains(&rock.y))
                .enumerate()
                .for_each(|(idx, rock)| {
                    rock.y = pillar_range.start + idx as i32;
                });
        }
    }
}

fn move_rocks_down(rocks: &mut [Rock], map: &Map) {
    rocks.sort_by(|a, b| match a.y.cmp(&b.y) {
        std::cmp::Ordering::Equal => a.x.cmp(&b.x),
        cmp => cmp,
    });

    for col in 0..map.cols {
        for pillar_range in map.vertical_ranges[col as usize].iter() {
            rocks
                .iter_mut()
                .filter(|rock| rock.x == col && pillar_range.contains(&rock.y))
                .enumerate()
                .for_each(|(idx, rock)| {
                    rock.y = pillar_range.end - idx as i32 - 1;
                });
        }
    }
}

fn move_rocks_left(rocks: &mut [Rock], map: &Map) {
    rocks.sort_by(|a, b| match a.x.cmp(&b.x) {
        std::cmp::Ordering::Equal => a.y.cmp(&b.y),
        cmp => cmp,
    });

    for row in 0..map.rows {
        for pillar_range in map.horizontal_ranges[row as usize].iter() {
            rocks
                .iter_mut()
                .filter(|rock| rock.y == row && pillar_range.contains(&rock.x))
                .enumerate()
                .for_each(|(idx, rock)| {
                    rock.x = pillar_range.start + idx as i32;
                });
        }
    }
}

fn move_rocks_right(rocks: &mut [Rock], map: &Map) {
    rocks.sort_by(|a, b| match a.x.cmp(&b.x) {
        std::cmp::Ordering::Equal => a.y.cmp(&b.y),
        cmp => cmp,
    });

    for row in 0..map.rows {
        for pillar_range in map.horizontal_ranges[row as usize].iter() {
            rocks
                .iter_mut()
                .filter(|rock| rock.y == row && pillar_range.contains(&rock.x))
                .enumerate()
                .for_each(|(idx, rock)| {
                    rock.x = pillar_range.end - idx as i32 - 1;
                });
        }
    }
}

fn part2(map: &Map, initial_rocks: &[Rock]) -> usize {
    let mut seen_states = HashMap::new();
    seen_states.insert(initial_rocks.to_owned(), 0);

    let mut curr_rock_positions = initial_rocks.to_owned();
    const DIRECTIONS: [&Dir; 4] = [UP, LEFT, DOWN, RIGHT];
    let states_before_cycle = loop {
        for dir in DIRECTIONS.iter() {
            move_rocks_smarter(dir, &mut curr_rock_positions, map);
        }

        if let Some(val) = seen_states.get(&curr_rock_positions) {
            break val;
        }
        seen_states.insert(curr_rock_positions.clone(), seen_states.len());
    };

    let cycle = seen_states.len() - states_before_cycle;
    let midx = (1000000000 - states_before_cycle).rem(cycle) + states_before_cycle;

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

    fn get_input<'a>() -> (Map, Vec<Rock>) {
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
