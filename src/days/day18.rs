use super::{ parse_file, parse_lines, pos::Pos, dir::DIRECTIONS};
use std::collections::{HashMap, VecDeque};

pub fn solve() {
    if let Ok(line_string) = parse_file("Inputs/day18.txt") {
        let lines = parse_lines(&line_string);
        let byte_positions = parse_byte_positions(&lines);
        let end_pos = Pos { x: 70, y: 70 };
        println!(
            "Part1 solution: {}",
            part1(&byte_positions[..1024], &end_pos)
        );
        use std::time::Instant;
        let now = Instant::now();
        println!("Part2 solution: {}", part2(&byte_positions, &end_pos));
        let elapsed = now.elapsed();
        println!("Elapsed: {:?}", elapsed);
    } else {
        println!("Could not parse file");
    }
}

fn parse_byte_positions(lines: &[String]) -> Vec<Pos> {
    let mut bytes = Vec::new();
    for line in lines.iter() {
        let mut num_str = line.split(",");
        let x = num_str.next().unwrap().parse::<i32>().unwrap();
        let y = num_str.next().unwrap().parse::<i32>().unwrap();
        bytes.push(Pos { x, y });
    }

    bytes
}

fn find_path(byte_positions: &[Pos], end_pos: &Pos) -> HashMap<Pos, usize> {
    let mut queue = VecDeque::new();
    let mut visited = HashMap::new();
    queue.push_back((Pos { x: 0, y: 0 }, 0_usize));

    while let Some((curr_pos, steps)) = queue.pop_front() {
        if let Some(old_steps) = visited.get_mut(&curr_pos) {
            if steps < *old_steps {
                *old_steps = steps;
            }
            continue;
        } else {
            visited.insert(curr_pos.clone(), steps);
        }

        for &direction in DIRECTIONS.iter() {
            let new_pos = &curr_pos + direction;

            if new_pos.x < 0 || new_pos.y < 0 || new_pos.x > end_pos.x || new_pos.y > end_pos.y {
                continue;
            }

            if byte_positions.contains(&new_pos) {
                continue;
            }

            queue.push_back((new_pos, steps + 1));
        }
    }

    visited
}

fn get_path(visited: &HashMap<Pos, usize>, end_pos: &Pos) -> Option<Vec<Pos>> {
    let mut path = Vec::new();

    visited.get(end_pos)?;

    let mut curr_pos = end_pos.clone();
    while curr_pos != (Pos { x: 0, y: 0 }) {
        path.push(curr_pos.clone());
        let mut min_steps = usize::MAX;
        let mut next_pos = Pos { x: 0, y: 0 };

        for &direction in DIRECTIONS.iter() {
            let new_pos = &curr_pos + direction;
            if let Some(steps) = visited.get(&new_pos) {
                if *steps < min_steps {
                    min_steps = *steps;
                    next_pos = new_pos;
                }
            }
        }

        curr_pos = next_pos;
    }

    Some(path)
}

fn part1(byte_positions: &[Pos], end_pos: &Pos) -> usize {
    let visited = find_path(byte_positions, end_pos);
    *visited.get(end_pos).unwrap()
}

fn part2(byte_positions: &[Pos], end_pos: &Pos) -> String {
    let mut i = 1;
    loop {
        let new_byte_positions = byte_positions[..i].to_vec();
        let visited = find_path(&new_byte_positions, end_pos);

        if let Some(path) = get_path(&visited, end_pos) {
            let mut j = i;
            while !path.contains(&byte_positions[j]) {
                j += 1;
            }
            i = j;
        } else {
            let str =
                byte_positions[i - 1].x.to_string() + "," + &byte_positions[i - 1].y.to_string();
            return str;
        }

        i += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_input() -> Vec<Pos> {
        let input = r#"5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0"#;

        let lines = parse_lines(&input);
        parse_byte_positions(&lines)
    }

    #[test]
    fn test_part1() -> Result<(), String> {
        let byte_positions = get_input();
        assert_eq!(part1(&byte_positions[..12], &Pos { x: 6, y: 6 }), 22);

        Ok(())
    }

    #[test]
    fn test_part2() -> Result<(), String> {
        let byte_positions = get_input();
        assert_eq!(part2(&byte_positions, &Pos { x: 6, y: 6 }), "6,1");

        Ok(())
    }
}
