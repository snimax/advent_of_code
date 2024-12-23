use advent_of_code_2024::{parse_file, parse_lines};
use std::collections::{HashMap, HashSet};

pub fn solve() {
    if let Ok(line_string) = parse_file("Inputs/day23.txt") {
        let lines = parse_lines(&line_string);
        let input = parse_graphs(&lines);
        println!("Part1 solution: {}", part1(&input));
        println!("Part2 solution: {}", part2(&input));
    } else {
        println!("Could not parse file");
    }
}

fn parse_graphs(input: &[String]) -> HashMap<String, Vec<String>> {
    let mut graph: HashMap<String, Vec<String>> = HashMap::new();

    let mut insert_into_graph = |a: &str, b: &str| {
        let a = a.to_string();
        let b = b.to_string();
        if let Some(ref mut val) = graph.get_mut(&a) {
            val.push(b);
        } else {
            graph.insert(a, vec![b]);
        }
    };

    for line in input {
        let a = &line[..2];
        let b = &line[(line.len() - 2)..];

        insert_into_graph(a, b);
        insert_into_graph(b, a);
    }

    graph
}

fn part1(graph: &HashMap<String, Vec<String>>) -> usize {
    let mut sets = HashSet::new();

    for (first_computer, first_computer_connections) in graph {
        for second_computer in first_computer_connections {
            if let Some(second_computer_connections) = graph.get(second_computer) {
                for third_computer in second_computer_connections {
                    if third_computer != first_computer {
                        if let Some(third_computer_connections) = graph.get(third_computer) {
                            if third_computer_connections.contains(first_computer) {
                                let combo1 = (first_computer, second_computer, third_computer);
                                let combo2 = (first_computer, third_computer, second_computer);
                                let combo3 = (second_computer, first_computer, third_computer);
                                let combo4 = (second_computer, third_computer, first_computer);
                                let combo5 = (third_computer, first_computer, second_computer);
                                let combo6 = (third_computer, second_computer, first_computer);
                                if !sets.contains(&combo1)
                                    && !sets.contains(&combo2)
                                    && !sets.contains(&combo3)
                                    && !sets.contains(&combo4)
                                    && !sets.contains(&combo5)
                                    && !sets.contains(&combo6)
                                {
                                    sets.insert(combo1);
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    sets.iter()
        .filter(|(a, b, c)| a.starts_with("t") || b.starts_with("t") || c.starts_with("t"))
        .count()
}

fn bron_kerbosh(
    current_clique: &HashSet<String>,
    possible_vertices_to_add: &HashSet<String>,
    already_processed_vertices: &HashSet<String>,
    graph: &HashMap<String, Vec<String>>,
    cliques: &mut Vec<HashSet<String>>,
) {
    if possible_vertices_to_add.is_empty() && already_processed_vertices.is_empty() {
        cliques.push(current_clique.clone());
    } else {
        let mut possible_vertices_to_add_clone = possible_vertices_to_add.clone();
        let mut already_processed_vertices = already_processed_vertices.clone();
        for vertex in possible_vertices_to_add {
            let mut new_current_clique = current_clique.clone();
            new_current_clique.insert(vertex.clone());
            if let Some(neighbors) = graph.get(vertex) {
                let neighbors = neighbors.iter().cloned().collect::<HashSet<String>>();
                let new_possible_vertices_to_add: HashSet<String> = possible_vertices_to_add_clone
                    .intersection(&neighbors)
                    .cloned()
                    .collect();
                let new_already_processed_vertices: HashSet<String> = already_processed_vertices
                    .intersection(&neighbors)
                    .cloned()
                    .collect();
                bron_kerbosh(
                    &new_current_clique,
                    &new_possible_vertices_to_add,
                    &new_already_processed_vertices,
                    graph,
                    cliques,
                );
                possible_vertices_to_add_clone.remove(vertex);
                already_processed_vertices.insert(vertex.clone());
            }
        }
    }
}

fn part2(graph: &HashMap<String, Vec<String>>) -> String {
    let possible_vertices_to_add: HashSet<String> = graph.keys().cloned().collect();
    let mut cliques = Vec::new();

    bron_kerbosh(
        &HashSet::new(),
        &possible_vertices_to_add,
        &HashSet::new(),
        graph,
        &mut cliques,
    );

    let mut largest_network_found = cliques
        .iter()
        .max_by_key(|clique| clique.len())
        .unwrap()
        .clone()
        .into_iter()
        .collect::<Vec<String>>();
    largest_network_found.sort();

    largest_network_found.join(",")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_input() -> HashMap<String, Vec<String>> {
        let input = r#"kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn"#;

        let lines = parse_lines(&input);
        parse_graphs(&lines)
    }

    #[test]
    fn test_part1() -> Result<(), String> {
        let input = get_input();
        assert_eq!(part1(&input), 7);

        Ok(())
    }

    #[test]
    fn test_part2() -> Result<(), String> {
        let input = get_input();
        assert_eq!(part2(&input), "co,de,ka,ta");

        Ok(())
    }
}
