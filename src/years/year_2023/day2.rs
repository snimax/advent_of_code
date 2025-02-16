use crate::years::AdventDay;

pub struct Day2 {}

impl AdventDay for Day2 {
    fn solve(&self) {
        let lines = self.get_input();
        let games = lines.iter().map(|line| parse_game(line)).collect::<Games>();
        println!("Part1 solution: {}", part1(&games));
        println!("Part2 solution: {}", part2(&games));
    }

    fn get_input_path(&self) -> &str {
        "Inputs/2023/day2.txt"
    }
}

struct Reveal {
    blue_cubes: u32,
    red_cubes: u32,
    green_cubes: u32,
}
type Reveals = Vec<Reveal>;

struct Game {
    id: u32,
    reveals: Reveals,
}
type Games = Vec<Game>;

fn parse_reveals(reveal_str: &str) -> Reveals {
    let parse_reveal = |reveal_str: &str| {
        let mut reveal = Reveal {
            blue_cubes: 0,
            red_cubes: 0,
            green_cubes: 0,
        };

        for subset in reveal_str.split(", ") {
            let mut colored_balls = subset.split_ascii_whitespace();
            let num = colored_balls.next().unwrap().parse::<u32>().unwrap();
            let color = colored_balls.last().unwrap();

            match color {
                "blue" => reveal.blue_cubes = num,
                "red" => reveal.red_cubes = num,
                "green" => reveal.green_cubes = num,
                _ => panic!("Invalid color"),
            }
        }

        reveal
    };

    let mut reveals = Reveals::new();

    for reveal in reveal_str.split(';') {
        reveals.push(parse_reveal(reveal));
    }

    reveals
}

fn parse_game(line: &str) -> Game {
    let mut game_str = line.split(':');
    let game_id = game_str
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .last()
        .unwrap()
        .parse::<u32>()
        .unwrap();

    let reveals = parse_reveals(game_str.next().unwrap_or(""));

    Game {
        id: game_id,
        reveals,
    }
}

fn game_is_possible(game: &Game) -> bool {
    const NUM_RED_CUBES: u32 = 12;
    const NUM_GREEN_CUBES: u32 = 13;
    const NUM_BLUE_CUBES: u32 = 14;

    game.reveals.iter().all(|reveal| {
        reveal.blue_cubes <= NUM_BLUE_CUBES
            && reveal.red_cubes <= NUM_RED_CUBES
            && reveal.green_cubes <= NUM_GREEN_CUBES
    })
}

fn part1(games: &[Game]) -> u32 {
    games
        .iter()
        .filter(|game| game_is_possible(game))
        .map(|game| game.id)
        .sum()
}

fn part2(games: &[Game]) -> u32 {
    games
        .iter()
        .map(|game| {
            let mut num_red_cubes = 0;
            let mut num_green_cubes = 0;
            let mut num_blue_cubes = 0;

            for reveal in &game.reveals {
                num_red_cubes = std::cmp::max(num_red_cubes, reveal.red_cubes);
                num_green_cubes = std::cmp::max(num_green_cubes, reveal.green_cubes);
                num_blue_cubes = std::cmp::max(num_blue_cubes, reveal.blue_cubes);
            }

            num_red_cubes * num_green_cubes * num_blue_cubes
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::library::*;

    fn get_games() -> Games {
        let input = r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"#;

        parse_lines(&input)
            .iter()
            .map(|line| parse_game(line))
            .collect::<Games>()
    }

    #[test]
    fn test_part1() -> Result<(), String> {
        let games = get_games();
        assert_eq!(part1(&games), 8);

        Ok(())
    }

    #[test]
    fn test_part2() -> Result<(), String> {
        let games = get_games();
        assert_eq!(part2(&games), 2286);

        Ok(())
    }
}
