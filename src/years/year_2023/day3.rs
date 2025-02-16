use crate::{
    dir::{DIAGONALS, DIRECTIONS},
    pos::Pos,
    years::AdventDay,
};
use std::ops::Range;

pub struct Day3 {}

impl AdventDay for Day3 {
    fn solve(&self) {
        let lines = self.get_input();
        let schematic = parse_schematic(&lines);
        println!("Part1 solution: {}", part1(&schematic));
        println!("Part2 solution: {}", part2(&schematic));
    }

    fn get_input_path(&self) -> &str {
        "Inputs/2023/day3.txt"
    }
}

#[derive(Debug)]
struct SchematicNumber {
    num: u32,
    row: i32,
    col: Range<i32>,
}

#[derive(Debug)]
struct Symbol {
    symbol: char,
    pos: Pos,
}

#[derive(Debug)]
struct Schematic {
    parts: Vec<SchematicNumber>,
    symbols: Vec<Symbol>,
}

fn parse_schematic(lines: &[String]) -> Schematic {
    let mut parts = Vec::new();
    let mut symbols = Vec::new();

    for (row, line) in lines.iter().enumerate() {
        let mut number = String::new();
        let mut start_idx = None;
        for (col, char) in line.chars().enumerate() {
            if char.is_ascii_digit() {
                if start_idx.is_none() {
                    start_idx = Some(col as i32);
                }
                number.push(char);
            } else if !number.is_empty() {
                parts.push(SchematicNumber {
                    num: number.parse().unwrap(),
                    row: row as i32,
                    col: start_idx.unwrap()..col as i32,
                });
                number.clear();
                start_idx = None;
            }
            if !char.is_ascii_digit() && char != '.' {
                symbols.push(Symbol {
                    symbol: char,
                    pos: Pos {
                        x: col as i32,
                        y: row as i32,
                    },
                });
            }
        }
        if let Some(start) = start_idx {
            parts.push(SchematicNumber {
                num: number.parse().unwrap(),
                row: row as i32,
                col: start..line.len() as i32,
            });
        }
    }

    Schematic { parts, symbols }
}

fn get_numbers_around_pos<'a>(
    schematic_numbers: &'a [SchematicNumber],
    pos: &Pos,
) -> Vec<&'a SchematicNumber> {
    schematic_numbers
        .iter()
        .filter(|number| {
            for &dir in DIRECTIONS.iter() {
                let new_pos = pos + dir;
                if number.row == new_pos.y && number.col.contains(&new_pos.x) {
                    return true;
                }
            }

            for &dir in DIAGONALS.iter() {
                let new_pos = pos + dir;
                if number.row == new_pos.y && number.col.contains(&new_pos.x) {
                    return true;
                }
            }
            false
        })
        .collect()
}

fn part1(schematic: &Schematic) -> u32 {
    schematic
        .symbols
        .iter()
        .map(|symbol| {
            let neighboring_numbers = get_numbers_around_pos(&schematic.parts, &symbol.pos);
            neighboring_numbers
                .iter()
                .map(|number| number.num)
                .sum::<u32>()
        })
        .sum()
}

fn part2(schematic: &Schematic) -> u32 {
    schematic
        .symbols
        .iter()
        .filter(|symbol| symbol.symbol == '*')
        .map(|symbol| get_numbers_around_pos(&schematic.parts, &symbol.pos))
        .filter(|neighboring_numbers| neighboring_numbers.len() == 2)
        .map(|neighboring_numbers| {
            neighboring_numbers
                .iter()
                .map(|number| number.num)
                .product::<u32>()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::library::*;

    fn get_input() -> Schematic {
        let input = r#"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."#;

        parse_schematic(&parse_lines(&input))
    }

    #[test]
    fn test_part1() -> Result<(), String> {
        let schematic = get_input();
        assert_eq!(part1(&schematic), 4361);

        Ok(())
    }

    #[test]
    fn test_part2() -> Result<(), String> {
        let schematic = get_input();
        assert_eq!(part2(&schematic), 467835);

        Ok(())
    }
}
