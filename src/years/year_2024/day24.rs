use super::{parse_file, parse_lines};
use std::collections::HashMap;

pub fn solve() {
    if let Ok(line_string) = parse_file("Inputs/day24.txt") {
        let lines = parse_lines(&line_string);
        let (wires, gates) = parse_wires(&lines);
        println!("Part1 solution: {}", part1(&wires, &gates));
        println!("Part2 solution: {}", part2(&wires, &gates));
    } else {
        println!("Could not parse file");
    }
}

type Wire = String;

#[derive(Debug, Clone, Copy, PartialEq, Hash)]
enum GateType {
    And,
    Or,
    Xor,
}

#[derive(Debug, Clone, PartialEq, Hash)]
struct Gate {
    op: GateType,
    a: Wire,
    b: Wire,
    res: Wire,
}

fn parse_wires(lines: &[String]) -> (HashMap<String, usize>, Vec<Gate>) {
    let mut wires = HashMap::new();
    let mut parsing_gates = false;
    let mut gates = Vec::new();

    for line in lines {
        if line.is_empty() {
            parsing_gates = true;
            continue;
        }

        if parsing_gates {
            let mut split = line.split_ascii_whitespace();
            let a = split.next().unwrap().to_string();
            let op = match split.next().unwrap() {
                "XOR" => GateType::Xor,
                "AND" => GateType::And,
                "OR" => GateType::Or,
                s => panic!("Got unexpected string '{s}' when trying to parse a gate"),
            };
            let b = split.next().unwrap().to_string();
            let res = split.nth(1).unwrap().to_string();

            gates.push(Gate { op, a, b, res });
        } else {
            let mut split = line.split(": ");
            wires.insert(
                split.next().unwrap().to_string(),
                split.next().unwrap().parse::<usize>().unwrap(),
            );
        }
    }

    (wires, gates)
}

fn part1(wires: &HashMap<String, usize>, gates: &[Gate]) -> usize {
    let new_wires = simulate_wires(wires, gates);
    get_wire_integer(&new_wires, "z")
}

fn simulate_wires(wires: &HashMap<String, usize>, gates: &[Gate]) -> HashMap<String, usize> {
    let mut wires = wires.clone();
    let mut gates = gates.to_owned();

    while !gates.is_empty() {
        let mut exit = true;
        for (idx, Gate { op, a, b, res }) in gates.iter().enumerate() {
            if let Some(wire_a) = wires.get(a) {
                if let Some(wire_b) = wires.get(b) {
                    let val = match op {
                        GateType::And => wire_a & wire_b,
                        GateType::Or => wire_a | wire_b,
                        GateType::Xor => wire_a ^ wire_b & 1,
                    };
                    wires.insert(res.clone(), val);
                    gates.remove(idx);
                    exit = false;
                    break;
                }
            }
        }
        if exit {
            break;
        }
    }

    wires
}

fn get_wire_integer(wires: &HashMap<String, usize>, name: &str) -> usize {
    let mut wires = wires
        .iter()
        .filter(|(wire, _)| wire.starts_with(name))
        .map(|(wire, val)| (wire.to_owned(), val.to_owned()))
        .collect::<Vec<(String, usize)>>();

    wires.sort();

    wires.iter().rev().fold(0, |mut acc, (_wire, val)| {
        acc <<= 1;
        acc | val
    })
}

fn has_input_wire(gate: &Gate) -> bool {
    (gate.a.starts_with("x") || gate.a.starts_with("y"))
        || (gate.b.starts_with("x") || gate.b.starts_with("y"))
}

fn is_least_significant_bit_input(gate: &Gate) -> bool {
    gate.a.ends_with("00") || gate.b.ends_with("00")
}

fn output_is_not_xor_gate(gate: &Gate) -> bool {
    gate.op != GateType::Xor && gate.res.starts_with("z") && gate.res != "z45"
}

fn xor_gate_not_input_nor_output(gate: &Gate) -> bool {
    gate.op == GateType::Xor && !gate.res.starts_with("z") && !has_input_wire(gate)
}

fn gate_appears_in_later_input(gate: &Gate, gates: &[Gate], op: GateType) -> bool {
    gates
        .iter()
        .any(|g| (g.a == gate.res || g.b == gate.res) && g.op == op)
}

fn is_falty_xor_gate(gate: &Gate, gates: &[Gate]) -> bool {
    gate.op == GateType::Xor
        && has_input_wire(gate)
        && !is_least_significant_bit_input(gate)
        && !gate_appears_in_later_input(gate, gates, GateType::Xor)
}

fn is_falty_and_gate(gate: &Gate, gates: &[Gate]) -> bool {
    gate.op == GateType::And
        && has_input_wire(gate)
        && !is_least_significant_bit_input(gate)
        && !gate_appears_in_later_input(gate, gates, GateType::Or)
}

fn part2(_wires: &HashMap<String, usize>, gates: &[Gate]) -> String {
    let mut falty_wires = Vec::new();

    for gate in gates.iter() {
        if output_is_not_xor_gate(gate)
            || xor_gate_not_input_nor_output(gate)
            || is_falty_and_gate(gate, gates)
            || is_falty_xor_gate(gate, gates)
        {
            falty_wires.push(gate.res.clone());
        }
    }

    falty_wires.sort();
    falty_wires.join(",")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_input_small() -> (HashMap<String, usize>, Vec<Gate>) {
        let input: &str = r#"x00: 1
x01: 1
x02: 1
y00: 0
y01: 1
y02: 0

x00 AND y00 -> z00
x01 XOR y01 -> z01
x02 OR y02 -> z02"#;

        let lines = parse_lines(&input);
        parse_wires(&lines)
    }

    fn get_input_large() -> (HashMap<String, usize>, Vec<Gate>) {
        let input: &str = r#"x00: 1
x01: 0
x02: 1
x03: 1
x04: 0
y00: 1
y01: 1
y02: 1
y03: 1
y04: 1

ntg XOR fgs -> mjb
y02 OR x01 -> tnw
kwq OR kpj -> z05
x00 OR x03 -> fst
tgd XOR rvg -> z01
vdt OR tnw -> bfw
bfw AND frj -> z10
ffh OR nrd -> bqk
y00 AND y03 -> djm
y03 OR y00 -> psh
bqk OR frj -> z08
tnw OR fst -> frj
gnj AND tgd -> z11
bfw XOR mjb -> z00
x03 OR x00 -> vdt
gnj AND wpb -> z02
x04 AND y00 -> kjc
djm OR pbm -> qhw
nrd AND vdt -> hwm
kjc AND fst -> rvg
y04 OR y02 -> fgs
y01 AND x02 -> pbm
ntg OR kjc -> kwq
psh XOR fgs -> tgd
qhw XOR tgd -> z09
pbm OR djm -> kpj
x03 XOR y03 -> ffh
x00 XOR y04 -> ntg
bfw OR bqk -> z06
nrd XOR fgs -> wpb
frj XOR qhw -> z04
bqk OR frj -> z07
y03 OR x01 -> nrd
hwm AND bqk -> z03
tgd XOR rvg -> z12
tnw OR pbm -> gnj"#;

        let lines = parse_lines(&input);
        parse_wires(&lines)
    }

    #[test]
    fn test_part1_small() -> Result<(), String> {
        let (wires, gates) = get_input_small();
        assert_eq!(part1(&wires, &gates), 4);

        Ok(())
    }

    #[test]
    fn test_part1_large() -> Result<(), String> {
        let (wires, gates) = get_input_large();
        assert_eq!(part1(&wires, &gates), 2024);

        Ok(())
    }
}
