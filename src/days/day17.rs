use advent_of_code_2024::{parse_file, parse_lines};

pub fn solve() {
    if let Ok(line_string) = parse_file("Inputs/day17.txt") {
        let lines = parse_lines(&line_string);
        let (registers, program) = parse_program(&lines);

        let (_, part1_solution) = part1(&registers, &program);
        let (registers, _) = part2(&program);

        println!("Part1 solution: {part1_solution}",);
        println!("Part2 solution: {}", registers.a);
    } else {
        println!("Could not parse file");
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Registers {
    a: u64,
    b: u64,
    c: u64,
}

type Program = Vec<u64>;

fn parse_program(lines: &[String]) -> (Registers, Program) {
    let mut registers = Registers { a: 0, b: 0, c: 0 };
    let mut program = Vec::new();

    for line in lines {
        if line.starts_with("Register A: ") {
            registers.a = line.split(": ").nth(1).unwrap().parse().unwrap();
        } else if line.starts_with("Register B: ") {
            registers.b = line.split(": ").nth(1).unwrap().parse().unwrap();
        } else if line.starts_with("Register C: ") {
            registers.c = line.split(": ").nth(1).unwrap().parse().unwrap();
        } else if line.starts_with("Program: ") {
            let program_str = line.split(": ").nth(1).unwrap();
            program = program_str
                .split(",")
                .map(|s| s.parse::<u64>().unwrap())
                .collect::<Vec<u64>>();
        }
    }

    (registers, program)
}

fn get_literal_operand_value(operand: u64) -> u64 {
    operand
}

fn get_combo_operand_value(operand: u64, registers: &Registers) -> u64 {
    if operand > 6 {
        panic!("Operand {operand:?} is not a combo operand!");
    }

    match operand {
        // Combo operands
        4 => registers.a,
        5 => registers.b,
        6 => registers.c,
        // Literal operands
        _ => operand,
    }
}

fn run_program(registers: &mut Registers, program: &Program) -> Vec<u64> {
    let mut output = Vec::new();

    let mut instruction_ptr = 0;
    while instruction_ptr < program.len() {
        let instruction = &program[instruction_ptr];
        let operand = &program[instruction_ptr + 1];
        match instruction {
            // adv
            0 => {
                let operand = get_combo_operand_value(*operand, registers);
                registers.a /= 2_u64.pow(operand as u32);
            }
            // bxl
            1 => {
                let operand = get_literal_operand_value(*operand);
                registers.b ^= operand;
            }
            // bst
            2 => {
                let operand = get_combo_operand_value(*operand, registers);
                registers.b = operand % 8;
            }
            // jnz
            3 => {
                let operand = get_literal_operand_value(*operand);

                if registers.a != 0 {
                    instruction_ptr = operand as usize;
                    continue;
                }
            }
            // bxc
            4 => {
                registers.b ^= registers.c;
            }
            // out
            5 => {
                let operand = get_combo_operand_value(*operand, registers);
                output.push(operand % 8);
            }
            // bdv
            6 => {
                let operand = get_combo_operand_value(*operand, registers);
                registers.b = registers.a / 2_u64.pow(operand as u32);
            }
            // cdv
            7 => {
                let operand = get_combo_operand_value(*operand, registers);
                registers.c = registers.a / 2_u64.pow(operand as u32);
            }
            _ => panic!("Unknown instruction {instruction:?}"),
        }

        instruction_ptr += 2;
    }

    output
}

fn part1(registers: &Registers, program: &Program) -> (Registers, String) {
    let mut registers = registers.clone();
    let output = run_program(&mut registers, program);

    let output_str = output
        .iter()
        .map(|i| i.to_string())
        .collect::<Vec<String>>()
        .join(",");

    (registers, output_str)
}

fn recurse(
    reverse_program_counter: usize,
    current_register_test: u64,
    program: &Program,
) -> Option<(u64, Vec<u64>)> {
    for i in 0..8 {
        let register_test_val = (current_register_test << 3) + i;
        let output = run_program(
            &mut Registers {
                a: register_test_val,
                b: 0,
                c: 0,
            },
            program,
        );

        if output == program[reverse_program_counter..] {
            if reverse_program_counter == 0 {
                return Some((register_test_val, output));
            }

            let result = recurse(reverse_program_counter - 1, register_test_val, program);
            if result.is_some() {
                return result;
            }
        }
    }

    None
}

fn part2(program: &Program) -> (Registers, String) {
    if let Some((val, output)) = recurse(program.len() - 1, 0, program) {
        let output_str = output
            .iter()
            .map(|i| i.to_string())
            .collect::<Vec<String>>()
            .join(",");

        return (Registers { a: val, b: 0, c: 0 }, output_str);
    }

    (Registers { a: 0, b: 0, c: 0 }, "".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_example() -> (Registers, Program) {
        let input = r#"Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0"#;

        let lines = parse_lines(&input);
        parse_program(&lines)
    }

    fn get_example1() -> (Registers, Program) {
        let input = r#"Register A: 0
Register B: 0
Register C: 9

Program: 2,6"#;

        let lines = parse_lines(&input);
        parse_program(&lines)
    }

    fn get_example2() -> (Registers, Program) {
        let input = r#"Register A: 10
Register B: 0
Register C: 0

Program: 5,0,5,1,5,4"#;

        let lines = parse_lines(&input);
        parse_program(&lines)
    }

    fn get_example3() -> (Registers, Program) {
        let input = r#"Register A: 2024
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0"#;

        let lines = parse_lines(&input);
        parse_program(&lines)
    }

    fn get_example4() -> (Registers, Program) {
        let input = r#"Register A: 0
Register B: 29
Register C: 0

Program: 1,7"#;

        let lines = parse_lines(&input);
        parse_program(&lines)
    }

    fn get_example5() -> (Registers, Program) {
        let input = r#"Register A: 0
Register B: 2024
Register C: 43690

Program: 4,0"#;

        let lines = parse_lines(&input);
        parse_program(&lines)
    }

    fn get_example6() -> (Registers, Program) {
        let input = r#"Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0"#;

        let lines = parse_lines(&input);
        parse_program(&lines)
    }

    #[test]
    fn test_part1() -> Result<(), String> {
        let (registers, program) = get_example();
        let (_registers, str) = part1(&registers, &program);
        assert_eq!(str, "4,6,3,5,6,3,5,2,1,0");

        Ok(())
    }

    #[test]
    fn test_part1_1() -> Result<(), String> {
        let (registers, program) = get_example1();
        let (registers, _str) = part1(&registers, &program);
        assert_eq!(registers.b, 1);

        Ok(())
    }

    #[test]
    fn test_part1_2() -> Result<(), String> {
        let (registers, program) = get_example2();
        let (_registers, str) = part1(&registers, &program);
        assert_eq!(str, "0,1,2");

        Ok(())
    }

    #[test]
    fn test_part1_3() -> Result<(), String> {
        let (registers, program) = get_example3();
        let (registers, str) = part1(&registers, &program);
        assert_eq!(registers.a, 0);
        assert_eq!(str, "4,2,5,6,7,7,7,7,3,1,0");

        Ok(())
    }

    #[test]
    fn test_part1_4() -> Result<(), String> {
        let (registers, program) = get_example4();
        let (registers, _str) = part1(&registers, &program);
        assert_eq!(registers.b, 26);

        Ok(())
    }

    #[test]
    fn test_part1_5() -> Result<(), String> {
        let (registers, program) = get_example5();
        let (registers, _str) = part1(&registers, &program);
        assert_eq!(registers.b, 44354);

        Ok(())
    }

    #[test]
    fn test_part2() -> Result<(), String> {
        let (_, program) = get_example6();
        let (registers, str) = part2(&program);
        assert_eq!(registers.a, 117440);
        assert_eq!(str, "0,3,5,4,3,0");

        Ok(())
    }
}
