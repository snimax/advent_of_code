use crate::{prime::PrimeFactorizationHelper, years::AdventDay};
use std::collections::{HashMap, HashSet};
pub struct Day8 {}

impl AdventDay for Day8 {
    fn solve(&self) {
        let lines = self.get_input();
        let (instructions, network) = parse_input(&lines);
        println!("Part1 solution: {}", part1(&instructions, &network));
        println!("Part2 solution: {}", part2(&instructions, &network));
    }

    fn get_input_path(&self) -> &str {
        "Inputs/2023/day8.txt"
    }
}

#[derive(Debug)]
enum Instruction {
    Left,
    Right,
}

type Node = String;

#[derive(Debug)]
struct Turn {
    left: Node,
    right: Node,
}

type Network = HashMap<Node, Turn>;

fn parse_input(lines: &[String]) -> (Vec<Instruction>, Network) {
    let instructions = lines
        .first()
        .expect("Instruction line")
        .chars()
        .map(|c| match c {
            'L' => Instruction::Left,
            'R' => Instruction::Right,
            c => panic!("Got unexpected char '{c}' when trying to parse the instruction string"),
        })
        .collect();

    let mut network = HashMap::new();
    for line in lines.iter().skip(2) {
        let node = line[0..3].to_owned();
        let left = line[7..10].to_owned();
        let right = line[12..15].to_owned();
        network.insert(node, Turn { left, right });
    }

    (instructions, network)
}

fn steps_to_finishing_pos(
    starting_nodes: &[Node],
    end_nodes: &[Node],
    instructions: &[Instruction],
    network: &Network,
) -> usize {
    let mut prime_cycles = HashSet::new();
    let mut prime_factorization_helper = PrimeFactorizationHelper::new();

    starting_nodes.iter().for_each(|node| {
        let mut curr_node = node.to_owned();
        for (steps, instruction) in instructions.iter().cycle().enumerate() {
            if end_nodes.contains(&curr_node) {
                let prime_factorizations = prime_factorization_helper.factorize(steps);
                for prime in prime_factorizations {
                    prime_cycles.insert(prime);
                }

                return;
            }
            if let Some(turn) = network.get(&curr_node) {
                curr_node = match instruction {
                    Instruction::Left => turn.left.clone(),
                    Instruction::Right => turn.right.clone(),
                }
            }
        }
    });

    prime_cycles.insert(instructions.len());
    prime_cycles.iter().product::<usize>()
}

fn part1(instructions: &[Instruction], network: &Network) -> usize {
    steps_to_finishing_pos(
        &["AAA".to_owned()],
        &["ZZZ".to_owned()],
        instructions,
        network,
    )
}

fn part2(instructions: &[Instruction], network: &Network) -> usize {
    let starting_nodes: Vec<Node> = network
        .keys()
        .filter(|node| node.ends_with('A'))
        .cloned()
        .collect();
    let end_nodes: Vec<Node> = network
        .keys()
        .filter(|node| node.ends_with('Z'))
        .cloned()
        .collect();
    steps_to_finishing_pos(&starting_nodes, &end_nodes, instructions, network)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::library::*;

    fn test_case_1() -> (Vec<Instruction>, Network) {
        let input = r#"RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)"#;

        parse_input(&parse_lines(&input))
    }

    fn test_case_2() -> (Vec<Instruction>, Network) {
        let input = r#"LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)"#;

        parse_input(&parse_lines(&input))
    }

    fn test_case_3() -> (Vec<Instruction>, Network) {
        let input = r#"LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)"#;

        parse_input(&parse_lines(&input))
    }

    #[test]
    fn test_part1_1() -> Result<(), String> {
        let (instructions, network) = test_case_1();
        assert_eq!(part1(&instructions, &network), 2);

        Ok(())
    }

    #[test]
    fn test_part1_2() -> Result<(), String> {
        let (instructions, network) = test_case_2();
        assert_eq!(part1(&instructions, &network), 6);

        Ok(())
    }

    #[test]
    fn test_part2() -> Result<(), String> {
        let (instructions, network) = test_case_3();
        assert_eq!(part2(&instructions, &network), 6);

        Ok(())
    }
}
