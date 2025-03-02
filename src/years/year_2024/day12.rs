use super::{Map, Pos};
use std::collections::{HashSet, VecDeque};

use crate::years::AdventDay;

pub struct Day12 {}

impl AdventDay for Day12 {
    fn solve(&self) {
        let lines = self.get_input();
        let map = parse_map(&lines);
        println!("Part1 solution: {}", part1(&map));
        println!("Part2 solution: {}", part2(&map));
    }

    fn get_input_path(&self) -> &str {
        "Inputs/2024/day12.txt"
    }
}

fn parse_map(lines: &[String]) -> Map<u8> {
    Map::new(lines, |char, _pos| char as u8)
}

fn map_region(map: &Map<u8>, start_pos: &Pos) -> (u8, HashSet<Pos>) {
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    let plant_type = map.get(start_pos);
    queue.push_back(start_pos.clone());
    while let Some(pos) = queue.pop_front() {
        if visited.insert(pos.clone()) {
            map.get_neighbors_cmp(&pos, plant_type)
                .iter()
                .for_each(|neighbor| queue.push_front(neighbor.clone()));
        }
    }
    (plant_type.to_owned(), visited)
}

fn parse_regions(map: &Map<u8>) -> Vec<(u8, HashSet<Pos>)> {
    let mut regions = Vec::new();

    for row in 0..map.size_y {
        for col in 0..map.size_x {
            let curr_pos = Pos {
                x: col as i32,
                y: row as i32,
            };
            if !regions
                .iter()
                .any(|(_, region): &(u8, HashSet<Pos>)| region.contains(&curr_pos))
            {
                regions.push(map_region(map, &curr_pos));
            }
        }
    }
    regions
}

fn part1(map: &Map<u8>) -> usize {
    let regions = parse_regions(map);

    let mut result = 0;
    for (plant_type, region) in regions.iter() {
        let mut perimiters = 0;
        for pos in region.iter() {
            perimiters += 4 - map.get_neighbors_cmp(pos, plant_type).len();
        }
        result += perimiters * region.len();
    }

    result
}

fn find_sides(region: &(u8, HashSet<Pos>), map: &Map<u8>) -> usize {
    let (plant_type, region) = region;

    let directions = [
        Pos { x: 0, y: -1 }, // N
        Pos { x: 1, y: 0 },  // E
        Pos { x: 0, y: 1 },  // S
        Pos { x: -1, y: 0 }, // W
    ]; // Right

    let diagonals = [
        Pos { x: 1, y: -1 },  // NE
        Pos { x: 1, y: 1 },   // SE
        Pos { x: -1, y: 1 },  // SW
        Pos { x: -1, y: -1 }, // NW
    ];

    let mut corners = 0;
    for position in region {
        let neighbors: Vec<bool> = directions
            .iter()
            .map(|p| {
                let pos = p + position;
                map.valid_pos(&pos) && *map.get(&pos) == *plant_type
            })
            .collect();

        let clockwise_neighbors = (neighbors[0], neighbors[1], neighbors[2], neighbors[3]);

        let diagonal_neighbors: Vec<bool> = diagonals
            .iter()
            .map(|p| {
                let pos = p + position;
                map.valid_pos(&pos) && *map.get(&pos) == *plant_type
            })
            .collect();

        let clockwise_diagonal_neighbors = (
            diagonal_neighbors[0],
            diagonal_neighbors[1],
            diagonal_neighbors[2],
            diagonal_neighbors[3],
        );

        let blah = corners;

        match clockwise_neighbors {
            // Island
            // ...
            // .A.
            // ...
            (false, false, false, false) => corners += 4,

            // One neighbor => edge of line
            // .A.
            // .A.
            // ...
            (true, false, false, false) => corners += 2,
            // ...
            // .AA
            // ...
            (false, true, false, false) => corners += 2,
            // ...
            // .A.
            // .A.
            (false, false, true, false) => corners += 2,
            // ...
            // AA.
            // ...
            (false, false, false, true) => corners += 2,

            // Clockwise neighbors => outside corner
            // .A?
            // .AA
            // ...
            (true, true, false, false) => {
                if clockwise_diagonal_neighbors.0 {
                    corners += 1
                } else {
                    corners += 2
                }
            }
            // ...
            // .AA
            // .A?
            (false, true, true, false) => {
                if clockwise_diagonal_neighbors.1 {
                    corners += 1
                } else {
                    corners += 2
                }
            }
            // ...
            // AA.
            // ?A.
            (false, false, true, true) => {
                if clockwise_diagonal_neighbors.2 {
                    corners += 1
                } else {
                    corners += 2
                }
            }
            // ?A.
            // AA.
            // ...
            (true, false, false, true) => {
                if clockwise_diagonal_neighbors.3 {
                    corners += 1
                } else {
                    corners += 2
                }
            }

            // Neighbors on each side
            // .A.
            // AAA
            // .A.
            (true, true, true, true) => {
                match clockwise_diagonal_neighbors {
                    (false, false, false, false) => corners += 4,
                    // One free pos around it
                    // AAA
                    // AAA
                    // .AA
                    (false, true, true, true) => corners += 1,
                    (true, false, true, true) => corners += 1,
                    (true, true, false, true) => corners += 1,
                    (true, true, true, false) => corners += 1,
                    // Two free pos around it
                    // AAA
                    // AAA
                    // .A.
                    (false, false, true, true) => corners += 2,
                    (true, false, false, true) => corners += 2,
                    (true, true, false, false) => corners += 2,
                    (false, true, true, false) => corners += 2,
                    // AA.
                    // AAA
                    // .AA
                    (false, true, false, true) => corners += 2,
                    (true, false, true, false) => corners += 2,
                    // Three free spots
                    // .A.
                    // AAA
                    // .AA
                    (true, false, false, false) => corners += 3,
                    (false, true, false, false) => corners += 3,
                    (false, false, true, false) => corners += 3,
                    (false, false, false, true) => corners += 3,

                    _ => {} // let other cases handle it
                }
            } // Handled by the edges

            // Inside corner
            // ...
            // AAA
            // ?A?
            (false, true, true, true) => match clockwise_diagonal_neighbors {
                (_, false, false, _) => corners += 2,
                (_, false, true, _) => corners += 1,
                (_, true, false, _) => corners += 1,
                _ => {}
            },
            // ?A.
            // AA.
            // ?A.
            (true, false, true, true) => match clockwise_diagonal_neighbors {
                (_, _, false, false) => corners += 2,
                (_, _, false, true) => corners += 1,
                (_, _, true, false) => corners += 1,
                _ => {}
            },
            // ?A?
            // AAA
            // ...
            (true, true, false, true) => match clockwise_diagonal_neighbors {
                (false, _, _, false) => corners += 2,
                (false, _, _, true) => corners += 1,
                (true, _, _, false) => corners += 1,
                _ => {}
            },
            // .A?
            // .AA
            // .A?
            (true, true, true, false) => match clockwise_diagonal_neighbors {
                (false, false, _, _) => corners += 2,
                (false, true, _, _) => corners += 1,
                (true, false, _, _) => corners += 1,
                _ => {}
            },

            // straight edges has no neighbors
            // ...
            // AAA
            // ...
            (true, false, true, false) => {}
            // .A.
            // .A.
            // .A.
            (false, true, false, true) => {}
        }

        if blah != corners {
            let diff = corners - blah;
            println!(
                "{diff} new corners with : {position:?} => {clockwise_neighbors:?}, {clockwise_diagonal_neighbors:?}"
            );
        } else {
            println!("{position:?} => {clockwise_neighbors:?}, {clockwise_diagonal_neighbors:?}");
        }
    }

    println!("{} => {}", *plant_type as char, corners);
    corners
}

fn part2(map: &Map<u8>) -> usize {
    let regions = parse_regions(map);

    let mut result = 0;
    for region in regions.iter() {
        let sides = find_sides(region, map);
        result += sides * region.1.len();
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::library::*;

    fn get_input() -> Map<u8> {
        let input = r#"AAAA
BBCD
BBCC
EEEC"#;

        let lines = parse_lines(&input);
        parse_map(&lines)
    }

    fn get_input1() -> Map<u8> {
        let input = r#"OOOOO
OXOXO
OOOOO
OXOXO
OOOOO"#;

        let lines = parse_lines(&input);
        parse_map(&lines)
    }

    fn get_input2() -> Map<u8> {
        let input = r#"RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE"#;

        let lines = parse_lines(&input);
        parse_map(&lines)
    }

    fn get_input3() -> Map<u8> {
        let input = r#"EEEEE
EXXXX
EEEEE
EXXXX
EEEEE"#;

        let lines = parse_lines(&input);
        parse_map(&lines)
    }

    fn get_input4() -> Map<u8> {
        let input = r#"AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA"#;

        let lines = parse_lines(&input);
        parse_map(&lines)
    }

    #[test]
    fn test_part1() -> Result<(), String> {
        let map = get_input();
        assert_eq!(part1(&map), 140);

        Ok(())
    }

    #[test]
    fn test_part1_1() -> Result<(), String> {
        let map = get_input1();
        assert_eq!(part1(&map), 772);

        Ok(())
    }

    #[test]
    fn test_part1_2() -> Result<(), String> {
        let map = get_input2();
        assert_eq!(part1(&map), 1930);

        Ok(())
    }

    #[test]
    fn test_part2() -> Result<(), String> {
        let map = get_input();
        assert_eq!(part2(&map), 80);

        Ok(())
    }

    #[test]
    fn test_part2_1() -> Result<(), String> {
        let map = get_input1();
        assert_eq!(part2(&map), 436);

        Ok(())
    }

    #[test]
    fn test_part2_2() -> Result<(), String> {
        let map = get_input2();
        assert_eq!(part2(&map), 1206);

        Ok(())
    }

    #[test]
    fn test_part2_3() -> Result<(), String> {
        let map = get_input3();
        assert_eq!(part2(&map), 236);

        Ok(())
    }

    #[test]
    fn test_part2_4() -> Result<(), String> {
        let map = get_input4();
        assert_eq!(part2(&map), 368);

        Ok(())
    }
}
