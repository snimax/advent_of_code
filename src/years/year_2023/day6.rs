use crate::years::AdventDay;
pub struct Day6 {}

impl AdventDay for Day6 {
    fn solve(&self) {
        let lines = self.get_input();
        let races = parse_input(&lines);
        println!("Part1 solution: {}", part1(&races));
        println!("Part2 solution: {}", part2(&races));
    }

    fn get_input_path(&self) -> &str {
        "Inputs/2023/day6.txt"
    }
}

#[derive(Debug)]
struct Race {
    time: u64,
    distance_to_beat: u64,
}

fn parse_input(lines: &[String]) -> Vec<Race> {
    let race_times = lines
        .iter()
        .next()
        .expect("Line with times is required")
        .split_ascii_whitespace()
        .skip(1) // Skip the "Time:" header
        .filter_map(|s| s.parse().ok())
        .collect::<Vec<u64>>();

    let distances_to_beat = lines
        .get(1)
        .expect("Line with distances is required")
        .split_ascii_whitespace()
        .skip(1) // Skip the "Distance:" header
        .filter_map(|s| s.parse().ok())
        .collect::<Vec<u64>>();

    race_times
        .iter()
        .zip(distances_to_beat.iter())
        .map(|(time, distance)| Race {
            time: *time,
            distance_to_beat: *distance,
        })
        .collect()
}

fn solve_race(race: &Race) -> u64 {
    let p = -(race.time as f64);
    let q = race.distance_to_beat as f64;
    let square_root = (p * p / 4.0 - q).sqrt();
    let lower_bound = f64::floor(-p / 2.0 - square_root) as u64;
    let upper_bound = f64::ceil(-p / 2.0 + square_root) as u64;
    upper_bound - lower_bound - 1
}

fn part1(races: &[Race]) -> u64 {
    races
        .iter()
        .fold(1, |acc, race: &Race| acc * solve_race(race))
}

fn part2(races: &[Race]) -> u64 {
    let mut time_str = String::new();
    let mut distance_str = String::new();

    for race in races {
        time_str.push_str(race.time.to_string().as_str());
        distance_str.push_str(race.distance_to_beat.to_string().as_str());
    }

    solve_race(&Race {
        time: time_str.parse().unwrap(),
        distance_to_beat: distance_str.parse().unwrap(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::library::*;

    fn get_lines() -> Vec<Race> {
        let input = r#"Time:      7  15   30
Distance:  9  40  200"#;

        parse_input(&parse_lines(&input))
    }

    #[test]
    fn test_part1() -> Result<(), String> {
        let races = get_lines();
        assert_eq!(part1(&races), 288);

        Ok(())
    }

    #[test]
    fn test_part2() -> Result<(), String> {
        let races = get_lines();
        assert_eq!(part2(&races), 71503);

        Ok(())
    }
}
