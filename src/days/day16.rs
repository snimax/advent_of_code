use advent_of_code_2024::{
    get_dir_name, parse_file, parse_lines, Dir, Map, Pos, DIRECTIONS, DOWN, LEFT, RIGHT, UP,
};
use std::collections::{HashMap, HashSet, VecDeque};

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
    visited.get(&end_pos).unwrap().1
}

fn get_neighbor_directions(pos: &Pos, map: &Map<Space>) -> Vec<Dir> {
    DIRECTIONS
        .iter()
        .filter(|dir| {
            let pos = pos.clone() + (*dir).clone();
            map.get(&pos) == Space::Empty
        })
        .map(|dir| dir.clone())
        .collect()
}

fn next_intersection_length(
    start_pos: &Pos,
    start_dir: &Dir,
    intersections: &HashSet<Pos>,
    map: &Map<Space>,
) -> (Pos, Dir, usize) {
    let mut queue = VecDeque::new();
    queue.push_back((start_pos.clone(), start_dir.clone(), 0));
    while let Some((curr_pos, curr_dir, steps)) = queue.pop_front() {
        if map.get(&curr_pos) == Space::Wall {
            continue;
        }

        if intersections.contains(&curr_pos) && steps > 0 {
            return (curr_pos, curr_dir, steps);
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

    panic!(
        "Got to unreachable pos from {start_pos:?}, with direction {}",
        get_dir_name(start_dir)
    );
}

type Graph = HashMap<Pos, HashMap<Dir, (Pos, Dir, usize)>>;

fn build_graph(start_pos: &Dir, end_pos: &Pos, map: &Map<Space>) -> Graph {
    let mut intersections = HashSet::new();
    // make sure start pos and end pos is in the set of intersections
    intersections.insert(start_pos.clone());
    intersections.insert(end_pos.clone());

    for row in 1..map.size_y - 1 {
        for col in 1..map.size_x - 1 {
            let curr_pos = Pos {
                x: row as i32,
                y: col as i32,
            };
            if map.get(&curr_pos) == Space::Empty
                && get_neighbor_directions(&curr_pos, map).len() != 2
            {
                intersections.insert(curr_pos);
            }
        }
    }

    let mut graph = Graph::new();

    for intersection in intersections.iter() {
        let neighbor_dirs = get_neighbor_directions(intersection, map);

        for neighbor_dir in neighbor_dirs.iter() {
            let (pos, dir, steps) =
                next_intersection_length(intersection, neighbor_dir, &intersections, map);

            if let Some(set) = graph.get_mut(&intersection) {
                set.insert(neighbor_dir.clone(), (pos.clone(), dir.clone(), steps));
            } else {
                let mut set = HashMap::new();
                set.insert(intersection.clone(), (pos.clone(), dir.clone(), steps));
                graph.insert(intersection.clone(), set);
            }

            if let Some(set) = graph.get_mut(&pos) {
                set.insert(
                    dir.clone(),
                    (intersection.clone(), neighbor_dir.clone(), steps),
                );
            } else {
                let mut set = HashMap::new();
                set.insert(dir, (intersection.clone(), neighbor_dir.clone(), steps));
                graph.insert(pos, set);
            }
        }
    }

    for (k, v) in graph.iter() {
        println!("{k:?}");
        println!("  {v:?}");
        println!();
    }
    graph
}

struct Goals {
    cost: usize,
    end: Pos,
}

fn find_path(
    cost: usize,
    curr_pos: &Pos,
    curr_dir: &Dir,
    goals: &Goals,
    graph: &Graph,
    used_paths: &mut HashSet<(Pos, Dir)>,
    in_optimal_paths: &mut HashSet<(Pos, Dir)>,
) -> bool {
    if *curr_pos == goals.end {
        return if cost == goals.cost {
            in_optimal_paths.insert((curr_pos.clone(), curr_dir.clone()));
            true
        } else { false };
    }

    if let Some(paths) = graph.get(curr_pos) {
        for (direction, (pos, dir, steps)) in paths {
            if used_paths.insert((curr_pos.clone(), direction.clone())) {
                let mut curr_cost = cost + steps;
                if direction != curr_dir {
                    curr_cost += 1000;
                }

                let optimal_path = find_path(curr_cost, pos, dir, goals, graph, used_paths, in_optimal_paths);
                if optimal_path {
                    in_optimal_paths.insert((pos.clone(), curr_dir.clone()));
                }
            }
        }
    }

    false
}

fn find_optimal_paths(start_pos: &Pos, goals: &Goals, graph: &Graph) -> usize {
    let mut used_paths = HashSet::new();
    let mut in_optimal_paths = HashSet::new();
    find_path(
        0,
        start_pos,
        &RIGHT,
        goals,
        graph,
        &mut used_paths,
        &mut in_optimal_paths,
    );

    println!("{in_optimal_paths:?}");
    0
}

fn part1(start_pos: &Pos, end_pos: &Pos, map: &Map<Space>) -> usize {
    find_straighetest_path(start_pos, &RIGHT, end_pos, map)
}

fn part2(start_pos: &Pos, end_pos: &Pos, map: &Map<Space>) -> usize {
    let optimal_path_cost = find_straighetest_path(start_pos, &RIGHT, end_pos, map);
    let graph = build_graph(start_pos, end_pos, map);
    let goals = Goals{cost: optimal_path_cost, end: end_pos.to_owned()};
    find_optimal_paths( start_pos, &goals, &graph)
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
