use advent_of_code_2024::{parse_file, parse_lines, Map, Pos, DOWN, LEFT, RIGHT, UP};
use std::collections::{HashSet, VecDeque};

pub fn solve() {
    if let Ok(line_string) = parse_file("Inputs/day10.txt") {
        let lines = parse_lines(&line_string);
        let (map, starting_positions) = parse_map(&lines);
        println!("Part1 solution: {}", part1(&map, &starting_positions));
        println!("Part2 solution: {}", part2(&map, &starting_positions));
    } else {
        println!("Could not parse file");
    }
}

fn parse_map(lines: &[String]) -> (Map<i32>, Vec<Pos>) {
    let size_y = lines.len();
    let size_x = lines[0].len();

    let mut map = vec![vec![]; size_y];
    let mut starting_positions = Vec::new();

    map.iter_mut().zip(lines).for_each(|(map_row, line)| {
        *map_row = line
            .chars()
            .map(|val| {
                if val.is_ascii_digit() {
                    val.to_string().parse::<i32>().unwrap()
                } else {
                    0
                }
            })
            .collect()
    });

    lines.iter().enumerate().for_each(|(row, line)| {
        line.as_bytes().iter().enumerate().for_each(|(col, val)| {
            if *val == b'0' {
                starting_positions.push(Pos {
                    x: col as i32,
                    y: row as i32,
                });
            }
        })
    });

    (
        Map {
            map,
            size_x,
            size_y,
        },
        starting_positions,
    )
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
            if map.get(&pos) == 9 {
                trailheads += 1;
            } else {
                get_neighbors(&pos, map)
                    .iter()
                    .for_each(|neighbor| queue.push_front(neighbor.clone()));
            }
        }
    }
    trailheads
}

fn get_neighbors(pos: &Pos, map: &Map<i32>) -> Vec<Pos> {
    let mut valid_neighbors = Vec::new();
    let next_height = map.get(pos) + 1;

    let mut next_pos = pos + UP;
    if map.valid_pos(&next_pos) && map.get(&next_pos) == next_height {
        valid_neighbors.push(next_pos);
    }

    next_pos = pos + DOWN;
    if map.valid_pos(&next_pos) && map.get(&next_pos) == next_height {
        valid_neighbors.push(next_pos);
    }

    next_pos = pos + LEFT;
    if map.valid_pos(&next_pos) && map.get(&next_pos) == next_height {
        valid_neighbors.push(next_pos);
    }

    next_pos = pos + RIGHT;
    if map.valid_pos(&next_pos) && map.get(&next_pos) == next_height {
        valid_neighbors.push(next_pos);
    }

    valid_neighbors
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
