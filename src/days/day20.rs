use advent_of_code_2024::{parse_file, parse_lines, Map, Pos, DIRECTIONS};
use std::collections::{HashMap, HashSet};

pub fn solve() {
    if let Ok(line_string) = parse_file("Inputs/day20.txt") {
        let lines = parse_lines(&line_string);
        let (start_pos, end_pos, map) = parse_map(&lines);
        println!("Part1 solution: {}", part1(&start_pos, &end_pos, &map, 100));
        println!("Part2 solution: {}", part2(&start_pos, &end_pos, &map));
    } else {
        println!("Could not parse file");
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

    let size_y = lines.len();
    let size_x = lines[0].len();
    let mut map = vec![vec![Space::Empty; size_x]; size_y];

    lines.iter().enumerate().for_each(|(row, line)| {
        line.chars().enumerate().for_each(|(col, c)| {
            map[row][col] = match c {
                '.' => Space::Empty,
                '#' => Space::Wall,
                'S' => {
                    start_pos = Pos {
                        x: col as i32,
                        y: row as i32,
                    };
                    Space::Empty
                }
                'E' => {
                    end_pos = Pos {
                        x: col as i32,
                        y: row as i32,
                    };
                    Space::Empty
                }
                c => panic!("Got unexpected char '{c}' when parsing map"),
            }
        });
    });

    (
        start_pos,
        end_pos,
        Map {
            map,
            size_x,
            size_y,
        },
    )
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
            let next_pos = curr_pos.clone() + dir;
            if map.get(&next_pos) != Space::Wall && !visited.contains(&next_pos) {
                curr_pos = next_pos;
                break;
            }
        }
    }

    path_length.push((end_pos.clone(), steps));
    let path_length = path_length.iter().rev().cloned().collect();
    path_length
}

fn part1(start_pos: &Pos, end_pos: &Pos, map: &Map<Space>, time_to_save: usize) -> usize {
    let path = find_path_length(end_pos, start_pos, map);
    println!("Time to beat: {:?}", path[0]);
    let lookup = path.iter().cloned().collect::<HashMap<Pos, usize>>();
    let mut shortcut_len = HashMap::new();
    for (pos, steps_to_end) in path.iter() {
        for dir in DIRECTIONS {
            let wall_pos = dir.clone() + pos.clone();
            let shortcut_pos = dir * 2 + pos.clone();
            if let Some(shortcut_len_to_end) = lookup.get(&shortcut_pos) {
                if map.get(&wall_pos) == Space::Wall && steps_to_end > shortcut_len_to_end {
                    let diff = steps_to_end - (shortcut_len_to_end + 2); // 2 for the steps walked during the cheat
                                                                         // println!("{pos:?}, {shortcut_pos:?}, {diff}");
                    if let Some(val) = shortcut_len.get_mut(&diff) {
                        *val += 1;
                    } else {
                        shortcut_len.insert(diff, 1);
                    }
                }
            }
        }
    }
    // println!("{shortcut_len:?}");

    for (k, v) in shortcut_len.iter() {
        println!("There are {v} cheats that save {k} picoseconds.");
    }

    shortcut_len.iter().fold(0, |acc, (k, v)| {
        acc + if *k >= time_to_save { *v } else { 0 }
    })
}

fn part2(_start_pos: &Pos, _end_pos: &Pos, _map: &Map<Space>) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

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

        Ok(())
    }

    #[test]
    fn test_part2() -> Result<(), String> {
        let (start_pos, end_pos, map) = get_map();
        assert_eq!(part2(&start_pos, &end_pos, &map), 31);

        Ok(())
    }
}
