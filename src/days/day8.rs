use advent_of_code_2024::{parse_file, parse_lines, Pos};
use std::{collections::HashMap, collections::HashSet};

pub fn solve() {
    if let Ok(line_string) = parse_file("Inputs/day8.txt") {
        let lines = parse_lines(&line_string);
        let (map_size, antenna_positions) = parse_map(&lines);
        println!("Part1 solution: {}", part1(&map_size, &antenna_positions));
        println!("Part2 solution: {}", part2(&map_size, &antenna_positions));
    } else {
        println!("Could not parse file");
    }
}

fn valid_pos(pos: &Pos, map_size: &Pos) -> bool {
    pos.x >= 0 && pos.y >= 0 && pos.x < map_size.x && pos.y < map_size.y
}

fn parse_map(lines: &[String]) -> (Pos, HashMap<char, Vec<Pos>>) {
    let map_size = Pos {
        x: lines[0].len() as i32,
        y: lines.len() as i32,
    };
    let mut antenna_positions: HashMap<char, Vec<Pos>> = HashMap::new();

    for (row, line) in lines.iter().enumerate() {
        for (col, _) in line
            .as_bytes()
            .iter()
            .enumerate()
            .filter(|(_, v)| **v != b'.')
        {
            let char = line.chars().nth(col).unwrap();
            let row = row as i32;
            let col = col as i32;
            if let Some(antennas) = antenna_positions.get_mut(&char) {
                antennas.push(Pos { x: col, y: row });
            } else {
                antenna_positions.insert(char, vec![Pos { x: col, y: row }]);
            }
        }
    }

    for (_, v) in antenna_positions.iter_mut() {
        v.sort();
    }

    (map_size, antenna_positions)
}

fn calculate_antinode_positions(
    map_size: &Pos,
    antenna_positions: &HashMap<char, Vec<Pos>>,
    part_two: bool,
) -> usize {
    let mut antinode_positions = HashSet::new();
    for (_, positions) in antenna_positions.iter() {
        for (idx, a) in positions.iter().enumerate() {
            for b in positions.iter().skip(idx + 1) {
                if part_two {
                    antinode_positions.insert(a.clone());
                    antinode_positions.insert(b.clone());
                }
                let diff = b - a;

                let mut antinode_pos = a - &diff;
                loop {
                    if valid_pos(&antinode_pos, map_size) {
                        antinode_positions.insert(antinode_pos.clone());
                    } else {
                        break;
                    }
                    if part_two {
                        antinode_pos = &antinode_pos - &diff;
                    } else {
                        break;
                    }
                }

                antinode_pos = b + &diff;
                loop {
                    if valid_pos(&antinode_pos, map_size) {
                        antinode_positions.insert(antinode_pos.clone());
                    } else {
                        break;
                    }
                    if part_two {
                        antinode_pos = &antinode_pos + &diff;
                    } else {
                        break;
                    }
                }
            }
        }
    }

    antinode_positions.len()
}

fn part1(map_size: &Pos, antenna_positions: &HashMap<char, Vec<Pos>>) -> usize {
    calculate_antinode_positions(map_size, antenna_positions, false)
}

fn part2(map_size: &Pos, antenna_positions: &HashMap<char, Vec<Pos>>) -> usize {
    calculate_antinode_positions(map_size, antenna_positions, true)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_equations() -> (Pos, HashMap<char, Vec<Pos>>) {
        let input = r#"............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............"#;

        let lines = parse_lines(&input);
        parse_map(&lines)
    }

    #[test]
    fn test_part1() -> Result<(), String> {
        let (map_size, antenna_positions) = get_equations();
        assert_eq!(part1(&map_size, &antenna_positions), 14);

        Ok(())
    }

    #[test]
    fn test_part2() -> Result<(), String> {
        let (map_size, antenna_positions) = get_equations();
        assert_eq!(part2(&map_size, &antenna_positions), 34);

        Ok(())
    }
}
