use crate::years::AdventDay;

use std::{cmp::Ordering, collections::HashMap};
pub struct Day7 {}

impl AdventDay for Day7 {
    fn solve(&self) {
        let lines = self.get_input();
        let hands = parse_input(&lines);
        println!("Part1 solution: {}", part1(&hands));
        println!("Part2 solution: {}", part2(&hands));
    }

    fn get_input_path(&self) -> &str {
        "Inputs/2023/day7.txt"
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Card(pub u8);

#[derive(Debug, Clone)]
struct Hand {
    cards: [Card; 5],
    strength: Strength,
    bid: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Strength {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

fn parse_strength(cards: &[Card]) -> Strength {
    let mut occurance_map = HashMap::new();

    for card in cards {
        occurance_map
            .entry(card)
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }

    let mut sorted_occurances = occurance_map.values().collect::<Vec<&i32>>();
    sorted_occurances.sort();
    sorted_occurances.reverse();

    let max_val = sorted_occurances[0];
    let second_highest = if sorted_occurances.len() > 1 {
        Some(sorted_occurances[1])
    } else {
        None
    };

    match max_val {
        5 => Strength::FiveOfAKind,
        4 => Strength::FourOfAKind,
        3 => {
            if second_highest.is_some_and(|val| *val == 2) {
                Strength::FullHouse
            } else {
                Strength::ThreeOfAKind
            }
        }
        2 => {
            if second_highest.is_some_and(|val| *val == 2) {
                Strength::TwoPair
            } else {
                Strength::OnePair
            }
        }
        1 => Strength::HighCard,
        num => panic!(
            "Got {} matching cards when trying to calculate the max occurances of the same card",
            num
        ),
    }
}

fn parse_rank(c: char) -> Card {
    if c.is_ascii_digit() {
        Card(c.to_digit(10).unwrap() as u8)
    } else {
        match c {
            'T' => Card(10),
            'J' => Card(11),
            'Q' => Card(12),
            'K' => Card(13),
            'A' => Card(14),
            _ => panic!("Invalid card rank"),
        }
    }
}

fn parse_input(lines: &[String]) -> Vec<Hand> {
    lines
        .iter()
        .map(|line| {
            let mut parts = line.split_ascii_whitespace();
            let hand_str = parts.next().expect("Hand of cards");
            let cards: [Card; 5] =
                core::array::from_fn(|i| parse_rank(hand_str.chars().nth(i).unwrap()));
            let bid = parts
                .next()
                .expect("Expected a bid")
                .parse()
                .expect("Expected a numerical bid");
            Hand {
                cards,
                strength: parse_strength(&cards),
                bid,
            }
        })
        .collect()
}

fn part1(hands: &[Hand]) -> usize {
    let mut hands = hands.to_vec();
    hands.sort_by(|lhs, rhs| {
        if lhs.strength != rhs.strength {
            lhs.strength.cmp(&rhs.strength)
        } else {
            for (lhs, rhs) in lhs.cards.iter().zip(rhs.cards.iter()) {
                if lhs != rhs {
                    // Intentional flip as higher card should be stronger than a lower card
                    return rhs.cmp(lhs);
                }
            }
            Ordering::Equal
        }
    });

    hands
        .iter()
        .rev()
        .enumerate()
        .map(|(idx, hand)| (idx + 1) * hand.bid as usize)
        .sum()
}

fn convert_strength_with_jokers(num_jokers: i32, hand: &Hand) -> Strength {
    if num_jokers == 0 || num_jokers == 5 {
        return hand.strength.to_owned();
    }

    let filtered_cards: Vec<Card> = hand
        .cards
        .iter()
        .filter(|card| **card != Card(1))
        .cloned()
        .collect();
    let strength_without_jokers = parse_strength(&filtered_cards);

    match strength_without_jokers {
        Strength::FiveOfAKind => Strength::FiveOfAKind,
        Strength::FourOfAKind => {
            Strength::FiveOfAKind
        }
        Strength::FullHouse => {
            Strength::FiveOfAKind
        }
        Strength::ThreeOfAKind => {
            match num_jokers {
                1 => Strength::FourOfAKind,
                2 => Strength::FiveOfAKind,
                num => panic!("Got {num} jokers when I Expected to only have 1 or 2 in Three of a kind case with hand: {:?}", hand.cards)
            }
        }
        Strength::TwoPair => {
            match num_jokers {
                1 => Strength::FullHouse,
                2 => Strength::FourOfAKind,
                num => panic!("Got {num} jokers when I expected to only have either 1 or 2 in a Two pair case with hand: {:?}", hand.cards)
            }
        },
        Strength::OnePair => {
            match num_jokers {
                1 => Strength::ThreeOfAKind,
                2 => Strength::FourOfAKind,
                3 => Strength::FiveOfAKind,
                num => panic!("Got {num} jokers when I expected to only have between 1 and 3 in a One pair case with hand: {:?}", hand.cards)
            }
        }
        Strength::HighCard => {
            match num_jokers {
                1 => Strength::OnePair,
                2 => Strength::ThreeOfAKind,
                3 => Strength::FourOfAKind,
                4 => Strength::FiveOfAKind,
                num => panic!("Got {num} jokers when I expected between 1 to 4 in HighCard case with hand: {:?}", hand.cards)
            }
        }
    }
}

fn convert_to_jokers(hands: &[Hand]) -> Vec<Hand> {
    let mut new_hands = hands.to_owned();
    for hand in new_hands.iter_mut() {
        let mut num_jokers = 0;
        for card in hand.cards.iter_mut() {
            if *card == Card(11) {
                *card = Card(1);
                num_jokers += 1;
            }
        }

        if num_jokers > 0 {
            hand.strength = convert_strength_with_jokers(num_jokers, hand);
        }
    }
    new_hands
}

fn part2(hands: &[Hand]) -> usize {
    let hands = convert_to_jokers(hands);
    part1(&hands)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::library::*;

    fn get_lines() -> Vec<Hand> {
        let input = r#"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483"#;

        parse_input(&parse_lines(&input))
    }

    #[test]
    fn test_part1() -> Result<(), String> {
        let hands = get_lines();
        assert_eq!(part1(&hands), 6440);

        Ok(())
    }

    #[test]
    fn test_part2() -> Result<(), String> {
        let hands = get_lines();
        assert_eq!(part2(&hands), 5905);

        Ok(())
    }
}
