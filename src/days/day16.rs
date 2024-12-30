use advent_of_code_2024::{
    get_opposite_dir, parse_file, parse_lines, Dir, Map, Pos, DIRECTIONS, DOWN, LEFT, RIGHT, UP,
};
use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap, HashSet, VecDeque},
    usize,
};

pub fn solve() {
    if let Ok(line_string) = parse_file("Inputs/day16.txt") {
        let lines = parse_lines(&line_string);
        let (start_pos, end_pos, map) = parse_map(&lines);
        println!("Part1 solution: {}", part1(&start_pos, &end_pos, &map));
        println!("Part2 solution: {}", part2(&start_pos, &end_pos, &map));
    } else {
        println!("Could not parse file");
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
enum Space {
    Wall,
    Empty,
}

fn parse_map(lines: &[String]) -> (Pos, Pos, Map<Space>) {
    let size_y = lines.len();
    let size_x = lines[0].len();
    let mut map = vec![vec![Space::Empty; size_x]; size_y];
    let mut start_pos = Pos { x: 0, y: 0 };
    let mut end_pos = Pos { x: 0, y: 0 };
    for (row, line) in lines.iter().enumerate() {
        for (col, char) in line.chars().enumerate() {
            match char {
                '#' => map[row][col] = Space::Wall,
                'S' => {
                    start_pos = Pos {
                        x: col as i32,
                        y: row as i32,
                    }
                }
                'E' => {
                    end_pos = Pos {
                        x: col as i32,
                        y: row as i32,
                    }
                }
                _ => {}
            }
        }
    }

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

fn rotate(dir: &Dir) -> (Dir, Dir) {
    if *dir == UP || *dir == DOWN {
        (LEFT, RIGHT)
    } else {
        (UP, DOWN)
    }
}

fn find_straighetest_path(
    curr_pos: &Pos,
    curr_dir: &Dir,
    end_pos: &Pos,
    map: &Map<Space>,
) -> usize {
    let mut queue = VecDeque::new();
    queue.push_back((curr_pos.clone(), curr_dir.clone(), 0));
    let mut visited = HashMap::new();

    while let Some((curr_pos, curr_dir, steps)) = queue.pop_front() {
        if let Some((dir, cached_steps)) = visited.get_mut(&curr_pos) {
            if steps < *cached_steps {
                *dir = curr_dir.clone();
                *cached_steps = steps;
            } else {
                // Should be no need to investigate this path
                continue;
            }
        } else {
            visited.insert(curr_pos.clone(), (curr_dir.clone(), steps));
        }

        let (cw, ccw) = rotate(&curr_dir);
        let neighbor_cw = curr_pos.clone() + cw.clone();
        let neighbor_ccw = curr_pos.clone() + ccw.clone();
        let next_pos = curr_pos.clone() + curr_dir.clone();

        if map.get(&next_pos) != Space::Wall {
            queue.push_back((next_pos, curr_dir.clone(), steps + 1));
        }
        if map.get(&neighbor_cw) != Space::Wall {
            queue.push_back((neighbor_cw, cw, steps + 1001));
        }
        if map.get(&neighbor_ccw) != Space::Wall {
            queue.push_back((neighbor_ccw, ccw, steps + 1001));
        }
    }

    visited.get(end_pos).unwrap().1
}

fn get_neighbor_directions(pos: &Pos, map: &Map<Space>) -> Vec<Dir> {
    DIRECTIONS
        .iter()
        .filter(|dir| {
            let pos = pos.clone() + (*dir).clone();
            map.get(&pos) == Space::Empty
        })
        .cloned()
        .collect()
}

fn _print_map(map: &Map<Space>, visited: &HashSet<Pos>, curr_pos: &Pos) {
    for row in 0..map.size_y {
        for col in 0..map.size_x {
            let pos = Pos {
                x: col as i32,
                y: row as i32,
            };
            if pos == *curr_pos {
                print!("X");
                continue;
            }
            match map.get(&pos) {
                Space::Wall => print!("#"),
                Space::Empty => {
                    if visited.contains(&pos) {
                        print!("O");
                    } else {
                        print!(".");
                    }
                }
            }
        }
        println!();
    }
    println!();
}

fn part1(start_pos: &Pos, end_pos: &Pos, map: &Map<Space>) -> usize {
    find_straighetest_path(start_pos, &RIGHT, end_pos, map)
}

fn part2(start_pos: &Pos, end_pos: &Pos, map: &Map<Space>) -> usize {
    let optimal_paths = find_optimal_paths(start_pos, end_pos, map);

    let mut unique_tiles = HashSet::new();
    for path in optimal_paths {
        for (tile, _) in path {
            unique_tiles.insert(tile);
        }
    }

    unique_tiles.len()
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Node {
    pos: Pos,
    dir: Dir,
    cost: usize,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn find_optimal_paths(start_pos: &Pos, end_pos: &Pos, map: &Map<Space>) -> Vec<Vec<(Pos, Dir)>> {
    let mut best_costs_found: HashMap<(Pos, Dir), usize> = HashMap::new();
    let mut predecessors: HashMap<(Pos, Dir), Vec<(Pos, Dir)>> = HashMap::new();
    let mut smallest_cost = usize::MAX;

    let mut queue = BinaryHeap::new();
    queue.push(Node {
        pos: start_pos.clone(),
        dir: RIGHT,
        cost: 0,
    });

    while let Some(Node {
        pos: curr_pos,
        dir: curr_dir,
        cost,
    }) = queue.pop()
    {
        if cost
            > *best_costs_found
                .get(&(curr_pos.clone(), curr_dir.clone()))
                .unwrap_or(&usize::MAX)
        {
            continue;
        }

        if curr_pos == *end_pos {
            smallest_cost = smallest_cost.min(cost);
        }

        for d in DIRECTIONS.iter() {
            let new_pos = curr_pos.clone() + d.clone();
            if map.get(&new_pos) == Space::Wall {
                continue;
            }

            let mut new_cost = cost + 1;
            let (cw, ccw) = rotate(&curr_dir);
            if *d == cw || *d == ccw {
                new_cost += 1000;
            }

            if new_cost > smallest_cost && curr_pos != *end_pos {
                continue;
            }

            if match best_costs_found.get(&(new_pos.clone(), d.clone())) {
                Some(cached_cost) => match new_cost.cmp(cached_cost) {
                    Ordering::Less => true,
                    Ordering::Equal => false,
                    Ordering::Greater => continue,
                },
                None => true,
            } {
                let key = (new_pos.clone(), d.clone());
                best_costs_found.insert(key.clone(), new_cost);
                predecessors.insert(key, vec![(curr_pos.clone(), curr_dir.clone())]);
            } else {
                if let Some(p) = predecessors.get_mut(&(new_pos.clone(), d.clone())) {
                    if !p.contains(&(curr_pos.clone(), curr_dir.clone())) {
                        p.push((curr_pos.clone(), curr_dir.clone()));
                    }
                }
            }

            queue.push(Node {
                pos: new_pos,
                dir: d.clone(),
                cost: new_cost,
            });
        }
    }

    let mut optimal_paths = Vec::new();
    let mut stack = VecDeque::new();

    for d in DIRECTIONS.iter() {
        if best_costs_found.contains_key(&(end_pos.clone(), d.clone())) {
            stack.push_back((
                vec![(end_pos.clone(), d.clone())],
                (end_pos.clone(), d.clone()),
            ));
        }
    }

    while let Some((curr_path, curr_node)) = stack.pop_back() {
        if curr_node == (start_pos.clone(), RIGHT) {
            let mut complete_path = curr_path.clone();
            complete_path.reverse();
            optimal_paths.push(complete_path);
        } else if let Some(prev_nodes) = predecessors.get(&curr_node) {
            for (prev_pos, prev_dir) in prev_nodes {
                let mut new_path = curr_path.clone();
                new_path.push((prev_pos.clone(), prev_dir.clone()));
                stack.push_back((new_path, (prev_pos.clone(), prev_dir.clone())));
            }
        }
    }

    optimal_paths
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_small_input() -> (Pos, Pos, Map<Space>) {
        let input = r#"###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############"#;

        let lines = parse_lines(&input);
        parse_map(&lines)
    }

    fn get_large_input() -> (Pos, Pos, Map<Space>) {
        let input = r#"#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################"#;

        let lines = parse_lines(&input);
        parse_map(&lines)
    }

    #[test]
    fn test_part1_small() -> Result<(), String> {
        let (start_pos, end_pos, map) = get_small_input();
        assert_eq!(part1(&start_pos, &end_pos, &map), 7036);

        Ok(())
    }

    #[test]
    fn test_part1_large() -> Result<(), String> {
        let (start_pos, end_pos, map) = get_large_input();
        assert_eq!(part1(&start_pos, &end_pos, &map), 11048);

        Ok(())
    }

    #[test]
    fn test_part2_small() -> Result<(), String> {
        let (start_pos, end_pos, map) = get_small_input();
        assert_eq!(part2(&start_pos, &end_pos, &map), 45);

        Ok(())
    }

    #[test]
    fn test_part2_large() -> Result<(), String> {
        let (start_pos, end_pos, map) = get_large_input();
        assert_eq!(part2(&start_pos, &end_pos, &map), 64);

        Ok(())
    }
}
