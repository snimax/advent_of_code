use crate::years::AdventDay;

use std::collections::HashSet;

use super::Pos3d;

pub struct Day8 {}

impl AdventDay for Day8 {
    fn solve(&self) {
        let lines = self.get_input();
        println!("Part1 solution: {}", part1(&lines, 1000));
        println!("Part2 solution: {}", part2(&lines));
    }

    fn get_input_path(&self) -> &str {
        "Inputs/2025/day8.txt"
    }
}

struct DistancePair {
    idx1: usize,
    idx2: usize,
    distance: i64,
}

fn parse_positions(lines: &[String]) -> Vec<Pos3d> {
    lines
        .iter()
        .map(|line| {
            let mut nums = line.split(',');
            if let Some(x) = nums.next().and_then(|s| s.parse().ok())
                && let Some(y) = nums.next().and_then(|s| s.parse().ok())
                && let Some(z) = nums.next().and_then(|s| s.parse().ok())
            {
                Pos3d { x, y, z }
            } else {
                panic!("Could not parse 3d position from '{line}'");
            }
        })
        .collect()
}

fn construct_distance_pairs(positions: &[Pos3d]) -> Vec<DistancePair> {
    let mut distance_pairs = Vec::with_capacity(positions.len() * positions.len());

    for (idx1, pos1) in positions.iter().enumerate() {
        for (idx2, pos2) in positions.iter().skip(idx1 + 1).enumerate() {
            let idx2 = idx2 + idx1 + 1;

            let distance_squared = i64::pow((pos1.x - pos2.x) as i64, 2)
                + i64::pow((pos1.y - pos2.y) as i64, 2)
                + i64::pow((pos1.z - pos2.z) as i64, 2);

            distance_pairs.push(DistancePair {
                idx1,
                idx2,
                distance: distance_squared,
            });
        }
    }

    distance_pairs.sort_by_key(|d| d.distance);
    distance_pairs
}

fn create_circuits(circuits: &mut Vec<HashSet<usize>>) {
    for (idx1, circuit1) in circuits.iter().enumerate() {
        for (idx2, circuit2) in circuits.iter().skip(idx1 + 1).enumerate() {
            let idx2 = idx2 + idx1 + 1;
            if circuit1.iter().any(|f| circuit2.contains(f)) {
                let mut combined_circuit = circuit1.clone();
                combined_circuit.extend(circuit2);
                circuits.remove(idx2);
                circuits.remove(idx1);
                circuits.push(combined_circuit);
                create_circuits(circuits);
                return;
            }
        }
    }
}

fn join_junction_boxes(
    distance_pairs: &[DistancePair],
    connections_to_make: usize,
) -> Vec<HashSet<usize>> {
    let mut circuits: Vec<HashSet<usize>> = Vec::new();

    distance_pairs.iter().take(connections_to_make).for_each(
        |&DistancePair {
             idx1,
             idx2,
             distance: _distance,
         }| {
            for circuit in circuits.iter_mut() {
                if circuit.contains(&idx1) || circuit.contains(&idx2) {
                    circuit.insert(idx1);
                    circuit.insert(idx2);
                    return;
                }
            }
            circuits.push(HashSet::from([idx1, idx2]));
        },
    );

    circuits
}

fn part1(lines: &[String], shortest_boxes_to_connect: usize) -> usize {
    let positions = parse_positions(lines);
    let distance_pairs = construct_distance_pairs(&positions);
    let mut circuits = join_junction_boxes(&distance_pairs, shortest_boxes_to_connect);
    create_circuits(&mut circuits);

    circuits.sort_by_key(|circuit| circuit.len());

    circuits
        .iter()
        .rev()
        .take(3)
        .map(|circuit| circuit.len())
        .product()
}

fn find_last_connection(positions: &[Pos3d], distance_pairs: &[DistancePair]) -> (usize, usize) {
    let mut circuits: Vec<HashSet<usize>> = Vec::new();

    for DistancePair {
        idx1,
        idx2,
        distance: _,
    } in distance_pairs
    {
        let mut inserted = false;
        for circuit in circuits.iter_mut() {
            if circuit.contains(idx1) || circuit.contains(idx2) {
                circuit.insert(*idx1);
                circuit.insert(*idx2);
                create_circuits(&mut circuits);

                inserted = true;
                break;
            }
        }
        if !inserted {
            circuits.push(HashSet::from([*idx1, *idx2]));
            create_circuits(&mut circuits);
        }

        if circuits.len() == 1 && circuits[0].len() == positions.len() {
            return (*idx1, *idx2);
        }
    }

    (0, 0)
}

fn part2(lines: &[String]) -> usize {
    let positions = parse_positions(lines);
    let distance_pairs = construct_distance_pairs(&positions);
    let (idx1, idx2) = find_last_connection(&positions, &distance_pairs);

    (positions[idx1].x as i64 * positions[idx2].x as i64) as usize
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::library::*;

    fn get_lines() -> Vec<String> {
        let input = r#"162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689"#;

        parse_lines(&input)
    }

    #[test]
    fn test_part1() -> Result<(), String> {
        let lines = get_lines();
        assert_eq!(part1(&lines, 10), 40);

        Ok(())
    }

    #[test]
    fn test_part2() -> Result<(), String> {
        let lines = get_lines();
        assert_eq!(part2(&lines), 25272);

        Ok(())
    }
}
