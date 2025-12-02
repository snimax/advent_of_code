use crate::years::AdventDay;
pub struct Day4 {}

impl AdventDay for Day4 {
    fn solve(&self) {
        let lines = self.get_input();
        let games = parse_games(&lines);
        println!("Part1 solution: {}", part1(&games));
        println!("Part2 solution: {}", part2(&games));
    }

    fn get_input_path(&self) -> &str {
        "Inputs/2023/day4.txt"
    }
}

struct Game {
    winning_numbers: Vec<u32>,
    numbers_you_have: Vec<u32>,
}

fn parse_game(line: &str) -> Game {
    let mut parts = line.split(':').next_back().unwrap().split(" | ");

    let winning_numbers = parts
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .map(|num| num.parse::<u32>().unwrap())
        .collect();

    let numbers_you_have = parts
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .map(|num| num.parse::<u32>().unwrap())
        .collect();

    Game {
        winning_numbers,
        numbers_you_have,
    }
}

fn parse_games(lines: &[String]) -> Vec<Game> {
    lines.iter().map(|line| parse_game(line)).collect()
}

fn get_winning_numbers(game: &Game) -> u32 {
    game.numbers_you_have
        .iter()
        .map(|num| game.winning_numbers.contains(num))
        .filter(|&v| v)
        .count() as u32
}

fn part1(games: &[Game]) -> u32 {
    games
        .iter()
        .map(|game| {
            let matches = get_winning_numbers(game);
            if matches > 1 {
                2_u32.pow(matches - 1)
            } else {
                matches
            }
        })
        .sum()
}

fn part2(games: &[Game]) -> u32 {
    let mut scratchcards = vec![1; games.len()];

    for (idx, game) in games.iter().enumerate() {
        let winning_numbers = get_winning_numbers(game);
        for i in idx + 1..idx + winning_numbers as usize + 1 {
            scratchcards[i] += scratchcards[idx];
        }
    }
    scratchcards.iter().sum::<u32>()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::library::*;

    fn get_lines() -> Vec<Game> {
        let input = r#"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"#;

        parse_games(&parse_lines(&input))
    }

    #[test]
    fn test_part1() -> Result<(), String> {
        let games = get_lines();
        assert_eq!(part1(&games), 13);

        Ok(())
    }

    #[test]
    fn test_part2() -> Result<(), String> {
        let games = get_lines();
        assert_eq!(part2(&games), 30);

        Ok(())
    }
}
