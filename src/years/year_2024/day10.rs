use super::{Map, Pos};
use std::collections::{HashSet, VecDeque};

use crate::years::AdventDay;

pub struct Day10 {}

impl AdventDay for Day10 {
    fn solve(&self) {
        let lines = self.get_input();
        let (map, starting_positions) = parse_map(&lines);
        println!("Part1 solution: {}", part1(&map, &starting_positions));
        println!("Part2 solution: {}", part2(&map, &starting_positions));
    }

    fn get_input_path(&self) -> &str {
        "Inputs/2024/day10.txt"
    }
}

fn parse_map(lines: &[String]) -> (Map<i32>, Vec<Pos>) {
    let mut starting_positions = Vec::new();
    let map = Map::new(lines, |val, pos| {
        if val.is_ascii_digit() {
            if val == '0' {
                starting_positions.push(pos.clone());
            }
            val.to_string().parse::<i32>().unwrap()
        } else {
            0
        }
    });

    (map, starting_positions)
}

fn part1(map: &Map<i32>, starting_positions: &[Pos]) -> usize {
    let mut result = 0;
    for start_pos in starting_positions.iter() {
        result += find_trailheads(map, start_pos, false);
    }

    result
}

fn find_trailheads(map: &Map<i32>, start_pos: &Pos, all_permutations: bool) -> usize {
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    let mut trailheads = 0;
    queue.push_back(start_pos.clone());
    while let Some(pos) = queue.pop_front() {
        if all_permutations || visited.insert(pos.clone()) {
            if *map.get(&pos) == 9 {
                trailheads += 1;
            } else {
                let new_height = map.get(&pos) + 1;
                map.get_neighbors_cmp(&pos, &new_height)
                    .iter()
                    .for_each(|neighbor| queue.push_front(neighbor.clone()));
            }
        }
    }
    trailheads
}

fn part2(map: &Map<i32>, starting_positions: &[Pos]) -> usize {
    let mut result = 0;
    for start_pos in starting_positions.iter() {
        result += find_trailheads(map, start_pos, true);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::library::*;

    fn get_input() -> (Map<i32>, Vec<Pos>) {
        let input = r#"89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732"#;

        let lines = parse_lines(&input);
        parse_map(&lines)
    }

    #[test]
    fn test_part1() -> Result<(), String> {
        let (map, starting_positions) = get_input();
        assert_eq!(part1(&map, &starting_positions), 36);

        Ok(())
    }

    fn get_input1() -> (Map<i32>, Vec<Pos>) {
        let input = r#"...0...
...1...
...2...
6543456
7.....7
8.....8
9.....9"#;

        let lines = parse_lines(&input);
        parse_map(&lines)
    }

    #[test]
    fn test_part1_2() -> Result<(), String> {
        let (map, starting_positions) = get_input1();
        assert_eq!(part1(&map, &starting_positions), 2);

        Ok(())
    }

    fn get_input2() -> (Map<i32>, Vec<Pos>) {
        let input = r#"..90..9
...1.98
...2..7
6543456
765.987
876....
987...."#;

        let lines = parse_lines(&input);
        parse_map(&lines)
    }

    #[test]
    fn test_part1_3() -> Result<(), String> {
        let (map, starting_positions) = get_input2();
        assert_eq!(part1(&map, &starting_positions), 4);

        Ok(())
    }

    fn get_input3() -> (Map<i32>, Vec<Pos>) {
        let input = r#"10..9..
2...8..
3...7..
4567654
...8..3
...9..2
.....01"#;

        let lines = parse_lines(&input);
        parse_map(&lines)
    }

    #[test]
    fn test_part1_4() -> Result<(), String> {
        let (map, starting_positions) = get_input3();
        assert_eq!(part1(&map, &starting_positions), 3);

        Ok(())
    }

    #[test]
    fn test_part2() -> Result<(), String> {
        let (map, starting_positions) = get_input();
        assert_eq!(part2(&map, &starting_positions), 81);

        Ok(())
    }

    fn get_input4() -> (Map<i32>, Vec<Pos>) {
        let input = r#".....0.
..4321.
..5..2.
..6543.
..7..4.
..8765.
..9...."#;

        let lines = parse_lines(&input);
        parse_map(&lines)
    }

    #[test]
    fn test_part2_1() -> Result<(), String> {
        let (map, starting_positions) = get_input4();
        assert_eq!(part2(&map, &starting_positions), 3);

        Ok(())
    }

    fn get_input5() -> (Map<i32>, Vec<Pos>) {
        let input = r#"..90..9
...1.98
...2..7
6543456
765.987
876....
987...."#;

        let lines = parse_lines(&input);
        parse_map(&lines)
    }

    #[test]
    fn test_part2_2() -> Result<(), String> {
        let (map, starting_positions) = get_input5();
        assert_eq!(part2(&map, &starting_positions), 13);

        Ok(())
    }

    fn get_input6() -> (Map<i32>, Vec<Pos>) {
        let input = r#"012345
123456
234567
345678
4.6789
56789."#;

        let lines = parse_lines(&input);
        parse_map(&lines)
    }

    #[test]
    fn test_part2_3() -> Result<(), String> {
        let (map, starting_positions) = get_input6();
        assert_eq!(part2(&map, &starting_positions), 227);

        Ok(())
    }
}
