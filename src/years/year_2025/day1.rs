use crate::years::AdventDay;

pub struct Day1 {}

impl AdventDay for Day1 {
    fn solve(&self) {
        let lines = self.get_input();
        println!("Part1 solution: {}", part1(&lines));
        println!("Part2 solution: {}", part2(&lines));
    }

    fn get_input_path(&self) -> &str {
        "Inputs/2025/day1.txt"
    }
}

#[derive(Debug)]
enum Direction {
    Right,
    Left,
}

#[derive(Debug)]
struct Rotation {
    dir: Direction,
    distance: i32,
}

fn parse_rotations(lines: &[String]) -> Vec<Rotation> {
    lines
        .iter()
        .map(|s| {
            let mut chars = s.chars();
            let c = chars.next().unwrap();

            let i = chars.as_str().parse().unwrap();

            match c {
                'R' => Rotation {
                    dir: Direction::Right,
                    distance: i,
                },
                'L' => Rotation {
                    dir: Direction::Left,
                    distance: i,
                },
                _ => panic!("Got unexpected char '{c}'"),
            }
        })
        .collect()
}

fn part1(lines: &[String]) -> u32 {
    let mut num_pointing_towards_0 = 0;
    let mut curr_dial_pos = 50;
    let rotations = parse_rotations(lines);
    for Rotation { dir, distance } in rotations {
        match dir {
            Direction::Right => curr_dial_pos = (curr_dial_pos + distance).rem_euclid(100),
            Direction::Left => curr_dial_pos = (curr_dial_pos - distance).rem_euclid(100),
        }

        if curr_dial_pos == 0 {
            num_pointing_towards_0 += 1;
        }
    }
    num_pointing_towards_0
}

fn part2(lines: &[String]) -> u32 {
    let mut num_pointing_towards_0 = 0;
    let mut curr_dial_pos = 50;
    let rotations = parse_rotations(lines);
    for Rotation { dir, distance } in rotations {
        let starting_at_zero = curr_dial_pos == 0;
        if distance > 100 {
            num_pointing_towards_0 += distance / 100;
        }

        let rem_distance = distance.rem_euclid(100);

        match dir {
            Direction::Right => curr_dial_pos += rem_distance,
            Direction::Left => curr_dial_pos -= rem_distance,
        }

        if curr_dial_pos == 0 && !starting_at_zero {
            num_pointing_towards_0 += 1;
        }

        if curr_dial_pos > 99 {
            num_pointing_towards_0 += 1;
            curr_dial_pos -= 100;
        } else if curr_dial_pos < 0 {
            if !starting_at_zero {
                num_pointing_towards_0 += 1;
            }
            curr_dial_pos += 100
        }
    }

    num_pointing_towards_0 as u32
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::library::*;

    fn get_lines() -> Vec<String> {
        let input = r#"L68
L30
R48
L5
R60
L55
L1
L99
R14
L82"#;

        parse_lines(&input)
    }

    fn get_loop_case() -> Vec<String> {
        let input = r#"R1000
L50"#;

        parse_lines(&input)
    }

    fn edge_case_right() -> Vec<String> {
        let input = r#"R150
L300
R101
L1"#;
        parse_lines(&input)
    }

    fn edge_case_left() -> Vec<String> {
        let input = r#"L250"#;
        parse_lines(&input)
    }

    #[test]
    fn test_part1() -> Result<(), String> {
        let lines = get_lines();
        assert_eq!(part1(&lines), 3);

        Ok(())
    }

    #[test]
    fn test_part2() -> Result<(), String> {
        let lines = get_lines();
        assert_eq!(part2(&lines), 6);

        Ok(())
    }

    #[test]
    fn test_part2_loop_case() -> Result<(), String> {
        let lines = get_loop_case();
        assert_eq!(part2(&lines), 11);

        Ok(())
    }

        #[test]
    fn test_part2_edge_case_left() -> Result<(), String> {
        let lines = edge_case_left();
        assert_eq!(part2(&lines), 3);

        Ok(())
    }

    #[test]
    fn test_part2_edge_case_right() -> Result<(), String> {
        let lines = edge_case_right();
        assert_eq!(part2(&lines), 7);

        Ok(())
    }
}
