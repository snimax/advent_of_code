use advent_of_code_2024::{parse_file, parse_lines};
use std::collections::HashMap;

pub fn solve() {
    if let Ok(line_string) = parse_file("Inputs/day1.txt") {
        let lines = parse_lines(&line_string);
        println!("Part1 solution: {}", part1(&lines));
        println!("Part2 solution: {}", part2(&lines));
    } else {
        println!("Could not parse file");
    }
}

fn part1(lines: &Vec<String>) -> i32 {
    let mut vec1 = Vec::new();
    let mut vec2 = Vec::new();

    for line in lines {
        let mut ids = line.split_ascii_whitespace();
        vec1.push(ids.next().unwrap().parse::<i32>().unwrap());
        vec2.push(ids.next().unwrap().parse::<i32>().unwrap());
    }

    vec1.sort();
    vec2.sort();

    vec1.iter()
        .zip(&vec2)
        .fold(0, |acc, (id1, id2)| acc + (id1 - id2).abs())
}

fn part2(lines: &Vec<String>) -> i32 {
    let mut numbers = Vec::new();
    let mut occurances = HashMap::new();

    for line in lines {
        let mut ids = line.split_ascii_whitespace();
        let id1 = ids.next().unwrap().parse::<i32>().unwrap();
        let id2 = ids.next().unwrap().parse::<i32>().unwrap();
        numbers.push(id1);

        if let Some(val) = occurances.get_mut(&id2) {
            *val += 1;
        } else {
            occurances.insert(id2, 1);
        }
    }

    numbers.iter().fold(0, |acc: i32, num| {
        acc + num * occurances.get(num).unwrap_or(&0)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_lines() -> Vec<String> {
        let input = r#"3   4
4   3
2   5
1   3
3   9
3   3"#;

        parse_lines(&input)
    }

    #[test]
    fn test_part1() -> Result<(), String> {
        let lines = get_lines();
        assert_eq!(part1(&lines), 11);

        Ok(())
    }

    #[test]
    fn test_part2() -> Result<(), String> {
        let lines = get_lines();
        assert_eq!(part2(&lines), 31);

        Ok(())
    }
}
