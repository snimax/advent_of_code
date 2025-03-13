use super::{Map, Pos};
use crate::years::AdventDay;

pub struct Day13 {}

impl AdventDay for Day13 {
    fn solve(&self) {
        let lines = self.get_input();
        let maps = parse_maps(&lines);
        println!("Part1 solution: {}", part1(&maps));
        println!("Part2 solution: {}", part2(&maps));
    }

    fn get_input_path(&self) -> &str {
        "Inputs/2023/day13.txt"
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Space {
    Ash,
    Rock,
}

fn parse_maps(lines: &[String]) -> Vec<Map<Space>> {
    let map_line_split = lines.split(|line| line.is_empty());

    map_line_split.map(|map_lines| Map::new(map_lines, |c, _|
        match c {
            '.' => Space::Ash,
            '#' => Space::Rock,
            _ => panic!("Got unexpected char '{c}' when trying to parse map. Expected map to only consist of '.' or '#'")
    })).collect()
}

fn num_row_differences(row1: usize, row2: usize, map: &Map<Space>) -> usize {
    (0..map.cols() as i32)
        .filter(|&x| map.get(&Pos { x, y: row1 as i32 }) != map.get(&Pos { x, y: row2 as i32 }))
        .count()
}

fn compare_row(row1: usize, row2: usize, map: &Map<Space>, smudge_fixed: bool) -> Option<usize> {
    if row1 >= map.rows() || row2 >= map.rows() {
        if smudge_fixed {
            return Some(0);
        } else {
            return None;
        }
    }

    let row_differences = num_row_differences(row1, row2, map);

    if !smudge_fixed && row_differences == 1 {
        if row1 > 0 {
            return compare_row(row1 - 1, row2 + 1, map, true).map(|v| v + 1);
        } else {
            return Some(1);
        }
    }

    if row_differences == 0 {
        if row1 > 0 {
            compare_row(row1 - 1, row2 + 1, map, smudge_fixed).map(|v| v + 1)
        } else if smudge_fixed {
            Some(1)
        } else {
            None
        }
    } else {
        None
    }
}

fn find_reflection_row(map: &Map<Space>, allow_smudge: bool) -> (usize, usize) {
    let mut best_row = 0;
    let mut val = 0;
    for row in 0..(map.rows() - 1) {
        let new_val = compare_row(row, row + 1, map, allow_smudge).unwrap_or(0);
        if new_val > val {
            val = new_val;
            best_row = row;
        }
    }
    (best_row, val)
}

fn find_best_reflection(map: &Map<Space>, allow_smudge: bool) -> usize {
    let smudge_fixed = !allow_smudge;
    let (row, row_val) = find_reflection_row(map, smudge_fixed);
    let mut transposed_map = map.clone();
    transposed_map.transpose();
    let (col, col_val) = find_reflection_row(&transposed_map, smudge_fixed);

    if row_val > col_val {
        (row + 1) * 100
    } else {
        col + 1
    }
}

fn part1(maps: &[Map<Space>]) -> usize {
    maps.iter()
        .map(|map| find_best_reflection(map, false))
        .sum()
}

fn part2(maps: &[Map<Space>]) -> usize {
    maps.iter().map(|map| find_best_reflection(map, true)).sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::library::*;

    #[test]
    fn test_part1() -> Result<(), String> {
        let maps = get_input();
        assert_eq!(part1(&maps), 405);

        Ok(())
    }

    #[test]
    fn test_part2() -> Result<(), String> {
        let maps = get_input();
        assert_eq!(part2(&maps), 400);

        Ok(())
    }

    fn get_input<'a>() -> Vec<Map<Space>> {
        let input = r#"#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#"#;

        parse_maps(&parse_lines(&input))
    }
}
