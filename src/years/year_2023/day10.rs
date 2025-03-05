use std::collections::VecDeque;

use super::{DIRECTIONS, DOWN, Dir, LEFT, Map, Pos, RIGHT, UP};
use crate::years::AdventDay;

pub struct Day10 {}

impl AdventDay for Day10 {
    fn solve(&self) {
        let lines = self.get_input();
        let (start_pos, map) = parse_map(&lines);
        println!("Part1 solution: {}", part1(&start_pos, &map));
        println!("Part2 solution: {}", part2(&start_pos, &map));
    }

    fn get_input_path(&self) -> &str {
        "Inputs/2023/day10.txt"
    }
}

#[derive(Debug, Clone)]
struct Pipe<'a> {
    directions: Option<(&'a Dir, &'a Dir)>,
    pos: Pos,
}

impl Pipe<'_> {
    fn is_connected_to(&self, pos: &Pos) -> bool {
        if let Some((dir1, dir2)) = self.directions {
            &self.pos + dir1 == *pos || &self.pos + dir2 == *pos
        } else {
            false
        }
    }
}

fn part1(start_pos: &Pos, map: &Map<Pipe>) -> usize {
    let pipe_loop = find_pipe_loop(start_pos, map);
    pipe_loop.len() / 2
}

fn part2(start_pos: &Pos, map: &Map<Pipe>) -> usize {
    let pipe_loop = find_pipe_loop(start_pos, map);

    let corners = pipe_loop
        .iter()
        .filter(|pos| {
            if let Some((dir1, dir2)) = map.get(pos).directions {
                return !dir1.is_opposite(dir2);
            }
            false
        })
        .collect::<Vec<&Pos>>();

    shoelace_formula(&corners) - pipe_loop.len()
}

fn find_pipe_loop(start_pos: &Pos, map: &Map<Pipe>) -> Vec<Pos> {
    let mut queue = VecDeque::from(vec![start_pos.clone()]);
    let mut visited = vec![start_pos.clone()];

    let mut first_iteration = true;

    while let Some(curr_pos) = queue.pop_front() {
        let pipe = map.get(&curr_pos);
        assert!(pipe.directions.is_some());

        let next_pos_0 = &curr_pos + pipe.directions.unwrap().0;
        if !visited.contains(&next_pos_0) {
            visited.push(next_pos_0.clone());
            queue.push_front(next_pos_0);
        }

        // Break tie to make sure visited is ordered
        if !first_iteration {
            let next_pos_1 = &curr_pos + pipe.directions.unwrap().1;
            if !visited.contains(&next_pos_1) {
                visited.push(next_pos_1.clone());
                queue.push_front(next_pos_1);
            }
        }
        first_iteration = false;
    }
    visited
}

fn shoelace_formula(pipe_loop: &Vec<&Pos>) -> usize {
    let (area, perimeter) = pipe_loop.iter().zip(pipe_loop.iter().cycle().skip(1)).fold(
        (0isize, 0isize),
        |(sum, perimeter), (p1, p2)| {
            let new_perimeter = perimeter + (p1.x.abs_diff(p2.x) + p1.y.abs_diff(p2.y)) as isize;
            let new_area = sum + ((p1.y * p2.x) as isize - (p1.x * p2.y) as isize);

            (new_area, new_perimeter)
        },
    );

    ((area.abs() + perimeter) / 2 + 1) as usize
}

fn parse_start_pos_type(start_pos: &Pos, map: &mut Map<Pipe>) {
    let mut pipes_connected_to_start = DIRECTIONS
        .iter()
        .filter(|dir| {
            if let Some(pipe) = map.next(start_pos, dir) {
                return pipe.is_connected_to(start_pos);
            }
            false
        })
        .collect::<Vec<&&Dir>>();

    assert!(
        pipes_connected_to_start.len() == 2,
        "Expected start pos to only be connected to two pipes!"
    );

    pipes_connected_to_start.sort();

    map.set(
        start_pos,
        Pipe {
            directions: Some((pipes_connected_to_start[0], pipes_connected_to_start[1])),
            pos: start_pos.clone(),
        },
    )
}

fn parse_map<'a>(lines: &[String]) -> (Pos, Map<Pipe<'a>>) {
    let mut start_pos = Pos { x: 0, y: 0 };

    let mut map = Map::new(lines, |char, pos| match char {
        '|' => Pipe {
            directions: Some((UP, DOWN)),
            pos: pos.clone(),
        },
        '-' => Pipe {
            directions: Some((LEFT, RIGHT)),
            pos: pos.clone(),
        },
        'L' => Pipe {
            directions: Some((UP, RIGHT)),
            pos: pos.clone(),
        },
        'J' => Pipe {
            directions: Some((UP, LEFT)),
            pos: pos.clone(),
        },
        '7' => Pipe {
            directions: Some((DOWN, LEFT)),
            pos: pos.clone(),
        },
        'F' => Pipe {
            directions: Some((DOWN, RIGHT)),
            pos: pos.clone(),
        },
        '.' => Pipe {
            directions: None,
            pos: pos.clone(),
        },
        'S' => {
            start_pos = pos.clone();
            Pipe {
                directions: None,
                pos: pos.clone(),
            }
        }
        _ => panic!("Got unexpected char '{char}' when trying to parse map!"),
    });

    parse_start_pos_type(&start_pos, &mut map);

    (start_pos, map)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::library::*;

    #[test]
    fn test_part1_loop() -> Result<(), String> {
        let (start_pos, map) = get_small_loop_map();
        assert_eq!(part1(&start_pos, &map), 4);

        Ok(())
    }

    #[test]
    fn test_part1_complicated() -> Result<(), String> {
        let (start_pos, map) = get_small_complicated_map();
        assert_eq!(part1(&start_pos, &map), 8);

        Ok(())
    }

    #[test]
    fn test_part2_loop() -> Result<(), String> {
        let (start_pos, map) = get_large_loop_map();
        assert_eq!(part2(&start_pos, &map), 4);

        Ok(())
    }

    #[test]
    fn test_part2_complicated() -> Result<(), String> {
        let (start_pos, map) = get_large_complicated_map();
        assert_eq!(part2(&start_pos, &map), 8);

        Ok(())
    }

    #[test]
    fn test_part2_complicated_2() -> Result<(), String> {
        let (start_pos, map) = get_large_complicated_map_2();
        assert_eq!(part2(&start_pos, &map), 10);

        Ok(())
    }

    fn get_small_loop_map<'a>() -> (Pos, Map<Pipe<'a>>) {
        let input = r#"-L|F7
7S-7|
L|7||
-L-J|
L|-JF"#;

        parse_map(&parse_lines(&input))
    }

    fn get_small_complicated_map<'a>() -> (Pos, Map<Pipe<'a>>) {
        let input = r#"..F7.
.FJ|.
SJ.L7
|F--J
LJ..."#;

        parse_map(&parse_lines(&input))
    }

    fn get_large_loop_map<'a>() -> (Pos, Map<Pipe<'a>>) {
        let input = r#"...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
..........."#;

        parse_map(&parse_lines(&input))
    }

    fn get_large_complicated_map<'a>() -> (Pos, Map<Pipe<'a>>) {
        let input = r#".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ..."#;

        parse_map(&parse_lines(&input))
    }

    fn get_large_complicated_map_2<'a>() -> (Pos, Map<Pipe<'a>>) {
        let input = r#"FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L"#;

        parse_map(&parse_lines(&input))
    }
}
