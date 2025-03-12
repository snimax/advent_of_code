use super::Pos;
use crate::years::AdventDay;

pub struct Day11 {}

impl AdventDay for Day11 {
    fn solve(&self) {
        let lines = self.get_input();
        let (galaxies, empty_space) = parse_map(&lines);
        println!("Part1 solution: {}", part1(&galaxies, &empty_space));
        println!("Part2 solution: {}", part2(&galaxies, &empty_space));
    }

    fn get_input_path(&self) -> &str {
        "Inputs/2023/day11.txt"
    }
}

#[derive(Debug)]
struct EmptySpace {
    rows: Vec<i32>,
    cols: Vec<i32>,
}

type Galaxy = Pos;

fn calculate_expanded_galaxy_distances(
    galaxies: &[Galaxy],
    empty_space: &EmptySpace,
    growth_factor: usize,
) -> usize {
    let mut sum = 0;
    let growth_factor = growth_factor - 1;
    for (idx, galaxy) in galaxies.iter().enumerate() {
        for galaxy2 in galaxies.iter().skip(idx + 1) {
            let min_x = galaxy.x.min(galaxy2.x);
            let max_x = galaxy.x.max(galaxy2.x);

            let min_y = galaxy.y.min(galaxy2.y);
            let max_y = galaxy.y.max(galaxy2.y);
            let empty_rows_passed = empty_space
                .rows
                .iter()
                .filter(|row| min_y < **row && **row < max_y)
                .count();
            let empty_cols_passed = empty_space
                .cols
                .iter()
                .filter(|col| min_x < **col && **col < max_x)
                .count();

            sum += galaxy.x.abs_diff(galaxy2.x) as usize
                + galaxy.y.abs_diff(galaxy2.y) as usize
                + growth_factor * (empty_cols_passed + empty_rows_passed);
        }
    }

    sum
}

fn part1(galaxies: &[Galaxy], empty_space: &EmptySpace) -> usize {
    calculate_expanded_galaxy_distances(galaxies, empty_space, 2)
}

fn part2(galaxies: &[Galaxy], empty_space: &EmptySpace) -> usize {
    calculate_expanded_galaxy_distances(galaxies, empty_space, 1000000)
}

fn parse_map(lines: &[String]) -> (Vec<Galaxy>, EmptySpace) {
    let mut galaxies = Vec::new();

    for (row, line) in lines.iter().enumerate() {
        for (col, c) in line.chars().enumerate() {
            let pos = Pos {
                x: col as i32,
                y: row as i32,
            };
            if c == '#' {
                galaxies.push(pos);
            }
        }
    }

    let rows = galaxies.iter().map(|galaxy| galaxy.y).max().unwrap();
    let cols = galaxies.iter().map(|galaxy| galaxy.x).max().unwrap();

    let rows = (0..=rows)
        .filter(|row| !galaxies.iter().any(|galaxy| galaxy.y == *row))
        .collect::<Vec<i32>>();
    let cols = (0..=cols)
        .filter(|col| !galaxies.iter().any(|galaxy| galaxy.x == *col))
        .collect::<Vec<i32>>();

    (galaxies, EmptySpace { rows, cols })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::library::*;

    #[test]
    fn test_part1() -> Result<(), String> {
        let (galaxies, empty_space) = get_input();
        assert_eq!(part1(&galaxies, &empty_space), 374);

        Ok(())
    }

    #[test]
    fn test_growth_factor_10() -> Result<(), String> {
        let (galaxies, empty_space) = get_input();
        assert_eq!(
            calculate_expanded_galaxy_distances(&galaxies, &empty_space, 10),
            1030
        );

        Ok(())
    }

    #[test]
    fn test_growth_factor_100() -> Result<(), String> {
        let (galaxies, empty_space) = get_input();
        assert_eq!(
            calculate_expanded_galaxy_distances(&galaxies, &empty_space, 100),
            8410
        );

        Ok(())
    }

    fn get_input<'a>() -> (Vec<Galaxy>, EmptySpace) {
        let input = r#"...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#....."#;

        parse_map(&parse_lines(&input))
    }
}
