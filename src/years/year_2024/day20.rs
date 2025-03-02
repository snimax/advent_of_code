use super::{DIRECTIONS, Map, Pos};
use std::collections::{HashMap, HashSet};

use crate::years::AdventDay;

pub struct Day20 {}

impl AdventDay for Day20 {
    fn solve(&self) {
        let lines = self.get_input();
        let (start_pos, end_pos, map) = parse_map(&lines);
        println!("Part1 solution: {}", part1(&start_pos, &end_pos, &map, 100));
        println!("Part2 solution: {}", part2(&start_pos, &end_pos, &map, 100));
    }

    fn get_input_path(&self) -> &str {
        "Inputs/2024/day20.txt"
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Space {
    Empty,
    Wall,
}

fn parse_map(lines: &[String]) -> (Pos, Pos, Map<Space>) {
    let mut start_pos = Pos { x: 0, y: 0 };
    let mut end_pos = Pos { x: 0, y: 0 };

    let map = Map::new(lines, |c, pos| match c {
        '.' => Space::Empty,
        '#' => Space::Wall,
        'S' => {
            start_pos = pos.clone();
            Space::Empty
        }
        'E' => {
            end_pos = pos.clone();
            Space::Empty
        }
        c => panic!("Got unexpected char '{c}' when parsing map"),
    });

    (start_pos, end_pos, map)
}

fn find_path_length(start_pos: &Pos, end_pos: &Pos, map: &Map<Space>) -> Vec<(Pos, usize)> {
    let mut visited = HashSet::new();
    let mut path_length = Vec::new();
    let mut curr_pos = start_pos.clone();
    let mut steps = 0;
    while curr_pos != *end_pos {
        visited.insert(curr_pos.clone());
        path_length.push((curr_pos.clone(), steps));
        steps += 1;
        for dir in DIRECTIONS {
            let next_pos = &curr_pos + dir;
            if *map.get(&next_pos) != Space::Wall && !visited.contains(&next_pos) {
                curr_pos = next_pos;
                break;
            }
        }
    }

    path_length.push((end_pos.clone(), steps));
    let path_length = path_length.iter().rev().cloned().collect();
    path_length
}

fn get_shortcuts(
    start_pos: &Pos,
    end_pos: &Pos,
    map: &Map<Space>,
    max_cheat_length: i32,
) -> HashMap<usize, usize> {
    let path = find_path_length(end_pos, start_pos, map);
    let lookup = path.iter().cloned().collect::<HashMap<Pos, usize>>();
    let mut shortcut_len = HashMap::new();
    let mut novel_cheats = HashSet::new();
    for (pos, steps_to_end) in path.iter() {
        for y in -max_cheat_length..=max_cheat_length {
            for x in -max_cheat_length..=max_cheat_length {
                let cheat_length = (i32::abs(x) + i32::abs(y)) as usize;
                if cheat_length > (max_cheat_length as usize) || cheat_length == 0 {
                    continue;
                }
                let shortcut_pos = pos + &Pos { x, y };

                if let Some(shortcut_len_to_end) = lookup.get(&shortcut_pos) {
                    if steps_to_end > shortcut_len_to_end {
                        let diff = steps_to_end - (shortcut_len_to_end + cheat_length);

                        if novel_cheats.insert((pos.clone(), shortcut_pos.clone())) {
                            if let Some(val) = shortcut_len.get_mut(&diff) {
                                *val += 1;
                            } else {
                                shortcut_len.insert(diff, 1);
                            }
                        }
                    }
                }
            }
        }
    }

    shortcut_len
}

fn part1(start_pos: &Pos, end_pos: &Pos, map: &Map<Space>, time_to_save: usize) -> usize {
    let shortcut_len = get_shortcuts(start_pos, end_pos, map, 2);

    shortcut_len.iter().fold(0, |acc, (k, v)| {
        acc + if *k >= time_to_save { *v } else { 0 }
    })
}

fn part2(start_pos: &Pos, end_pos: &Pos, map: &Map<Space>, time_to_save: usize) -> usize {
    let shortcut_len = get_shortcuts(start_pos, end_pos, map, 20);

    shortcut_len.iter().fold(0, |acc, (k, v)| {
        acc + if *k >= time_to_save { *v } else { 0 }
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::library::*;

    fn get_map() -> (Pos, Pos, Map<Space>) {
        let input = r#"###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############"#;

        let lines = parse_lines(&input);
        parse_map(&lines)
    }

    #[test]
    fn test_part1() -> Result<(), String> {
        let (start_pos, end_pos, map) = get_map();
        assert_eq!(part1(&start_pos, &end_pos, &map, 2), 44);
        assert_eq!(part1(&start_pos, &end_pos, &map, 20), 5);

        Ok(())
    }

    #[test]
    fn test_part2() -> Result<(), String> {
        let (start_pos, end_pos, map) = get_map();
        assert_eq!(part2(&start_pos, &end_pos, &map, 50), 285);

        Ok(())
    }
}
