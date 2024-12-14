use advent_of_code_2024::{parse_file, parse_lines, Dir, Pos};

pub fn solve() {
    if let Ok(line_string) = parse_file("Inputs/day14.txt") {
        let lines = parse_lines(&line_string);
        let robots = parse_map(&lines);
        let map_size = Pos { x: 101, y: 103 };
        println!("Part1 solution: {}", part1(&map_size, &robots));
        println!("Part2 solution: {}", part2(&map_size, &robots));
    } else {
        println!("Could not parse file");
    }
}

#[derive(Debug, Clone)]
struct Robot {
    pos: Pos,
    dir: Dir,
}

fn parse_vec2(str: &str) -> (i32, i32) {
    let mut split = str.split(',');

    let x = split.next().unwrap().parse::<i32>().unwrap();
    let y = split.next().unwrap().parse::<i32>().unwrap();

    (x, y)
}

fn parse_map(lines: &[String]) -> Vec<Robot> {
    let mut robots = Vec::new();

    for line in lines.iter() {
        let mut slices = line.split_ascii_whitespace();

        let pos_str = slices.next().unwrap().trim_start_matches("p=");
        let (pos_x, pos_y) = parse_vec2(pos_str);

        let dir_str = slices.next().unwrap().trim_start_matches("v=");
        let (dir_x, dir_y) = parse_vec2(dir_str);

        robots.push(Robot {
            pos: Pos { x: pos_x, y: pos_y },
            dir: Dir { x: dir_x, y: dir_y },
        })
    }

    robots
}

fn is_upper_range(x: i32, divider: i32) -> Option<bool> {
    use std::cmp::Ordering;
    match divider.cmp(&x) {
        Ordering::Less => Some(false),
        Ordering::Equal => None,
        Ordering::Greater => Some(true),
    }
}

fn part1(map_size: &Pos, robots: &[Robot]) -> usize {
    let mut final_positions = Vec::new();
    for robot in robots {
        let moved_pos = robot.pos.clone() + robot.dir.clone() * 100;

        let mut new_x = moved_pos.x % map_size.x;
        let mut new_y = moved_pos.y % map_size.y;
        if new_x < 0 {
            new_x += map_size.x;
        }
        if new_y < 0 {
            new_y += map_size.y;
        }

        final_positions.push(Pos { x: new_x, y: new_y });
    }

    let mut quadrants = [0; 4];

    let x_divider = map_size.x / 2;
    let y_divider = map_size.y / 2;

    for pos in final_positions {
        if let (Some(top_side), Some(right_side)) = (
            is_upper_range(pos.x, x_divider),
            is_upper_range(pos.y, y_divider),
        ) {
            match (top_side, right_side) {
                (false, false) => quadrants[0] += 1,
                (true, false) => quadrants[1] += 1,
                (true, true) => quadrants[2] += 1,
                (false, true) => quadrants[3] += 1,
            }
        }
    }

    quadrants.iter().fold(1, |acc, &e| acc * e as usize)
}

fn found_multiple_robots_in_a_row(robots: &[Robot]) -> bool {
    let required_robots_in_a_row = 10;
    let mut sorted_robots = robots.to_vec();

    sorted_robots.sort_by(|lhs, rhs| {
        let y_res = lhs.pos.y.cmp(&rhs.pos.y);
        if y_res != std::cmp::Ordering::Equal {
            y_res
        } else {
            lhs.pos.x.cmp(&rhs.pos.x)
        }
    });

    let mut curr_col = sorted_robots[0].pos.x;
    let mut curr_row = sorted_robots[0].pos.y;
    let mut robots_in_row = 1;
    for robot in sorted_robots.iter().skip(1) {
        if robot.pos.y == curr_row {
            if robot.pos.x == curr_col + 1 {
                robots_in_row += 1;
                curr_col += 1;
            } else if robots_in_row >= required_robots_in_a_row {
                return true;
            }
        } else if robots_in_row >= required_robots_in_a_row {
            return true;
        } else {
            curr_col = robot.pos.x;
            curr_row = robot.pos.y;
            robots_in_row = 1;
        }
    }
    robots_in_row >= required_robots_in_a_row
}

fn part2(map_size: &Pos, robots: &[Robot]) -> usize {
    let mut curr_position_robots = robots.to_vec();

    let mut steps = 0;
    let steps_for_repeating_pattern = map_size.x as usize * map_size.y as usize;
    while !found_multiple_robots_in_a_row(&curr_position_robots)
        && steps < steps_for_repeating_pattern
    {
        for robot in curr_position_robots.iter_mut() {
            let new_pos = robot.pos.clone() + robot.dir.clone();
            let mut new_x = new_pos.x % map_size.x;
            let mut new_y = new_pos.y % map_size.y;
            if new_x < 0 {
                new_x += map_size.x;
            }
            if new_y < 0 {
                new_y += map_size.y;
            }
            robot.pos = Pos { x: new_x, y: new_y };
        }
        steps += 1;
    }

    for y in 0..map_size.y {
        for x in 0..map_size.x {
            if curr_position_robots
                .iter()
                .any(|robot| robot.pos == Pos { x, y })
            {
                print!("X");
            } else {
                print!(".");
            }
        }
        println!();
    }
    steps
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_robots() -> (Pos, Vec<Robot>) {
        let input = r#"p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3"#;

        let lines = parse_lines(&input);
        (Pos { x: 11, y: 7 }, parse_map(&lines))
    }

    #[test]
    fn test_part1() -> Result<(), String> {
        let (map_size, robots) = get_robots();
        assert_eq!(part1(&map_size, &robots), 12);

        Ok(())
    }

    #[test]
    fn robots_in_row() -> Result<(), String> {
        let robots = [
            Robot {
                pos: Pos { x: 0, y: 0 },
                dir: Pos { x: 0, y: 0 },
            },
            Robot {
                pos: Pos { x: 4, y: 2 },
                dir: Pos { x: 0, y: 0 },
            },
            Robot {
                pos: Pos { x: 1, y: 0 },
                dir: Pos { x: 0, y: 0 },
            },
            Robot {
                pos: Pos { x: 0, y: 1 },
                dir: Pos { x: 0, y: 0 },
            },
            Robot {
                pos: Pos { x: 1, y: 1 },
                dir: Pos { x: 0, y: 0 },
            },
            Robot {
                pos: Pos { x: 3, y: 1 },
                dir: Pos { x: 0, y: 0 },
            },
            Robot {
                pos: Pos { x: 0, y: 2 },
                dir: Pos { x: 0, y: 0 },
            },
            Robot {
                pos: Pos { x: 1, y: 2 },
                dir: Pos { x: 0, y: 0 },
            },
            Robot {
                pos: Pos { x: 2, y: 2 },
                dir: Pos { x: 0, y: 0 },
            },
            Robot {
                pos: Pos { x: 3, y: 2 },
                dir: Pos { x: 0, y: 0 },
            },
            Robot {
                pos: Pos { x: 5, y: 2 },
                dir: Pos { x: 0, y: 0 },
            },
            Robot {
                pos: Pos { x: 6, y: 2 },
                dir: Pos { x: 0, y: 0 },
            },
            Robot {
                pos: Pos { x: 7, y: 2 },
                dir: Pos { x: 0, y: 0 },
            },
            Robot {
                pos: Pos { x: 8, y: 2 },
                dir: Pos { x: 0, y: 0 },
            },
            Robot {
                pos: Pos { x: 9, y: 2 },
                dir: Pos { x: 0, y: 0 },
            },
            Robot {
                pos: Pos { x: 10, y: 2 },
                dir: Pos { x: 0, y: 0 },
            },
            Robot {
                pos: Pos { x: 11, y: 2 },
                dir: Pos { x: 0, y: 0 },
            },
        ];
        assert_eq!(found_multiple_robots_in_a_row(&robots), true);

        Ok(())
    }
}
