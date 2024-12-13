use advent_of_code_2024::{cramers_rule, parse_file, parse_lines, Equation, Pos};

pub fn solve() {
    if let Ok(line_string) = parse_file("Inputs/day13.txt") {
        let lines = parse_lines(&line_string);
        let claw_machines = parse_claw_machines(&lines);
        println!("Part1 solution: {}", part1(&claw_machines));
        println!("Part2 solution: {}", part2(&claw_machines));
    } else {
        println!("Could not parse file");
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct ClawMachine {
    button_a: Pos,
    button_b: Pos,
    prize: Pos,
}

fn parse_button(line: &str) -> Pos {
    let (_, pos_str) = line.split_at(10);

    let mut pos_slices = pos_str.split(',');

    let x_str = pos_slices.next().unwrap().trim_start_matches("X+");
    let x = x_str.parse::<i32>().unwrap();
    let y_str = pos_slices.next().unwrap().trim_start_matches(" Y+");
    let y = y_str.parse::<i32>().unwrap();

    Pos { x, y }
}

fn parse_prize(line: &str) -> Pos {
    let (_, pos_str) = line.split_at(7);

    let mut pos_slices = pos_str.split(',');

    let x = pos_slices
        .next()
        .unwrap()
        .trim_start_matches("X=")
        .parse::<i32>()
        .unwrap();
    let y = pos_slices
        .next()
        .unwrap()
        .trim_start_matches(" Y=")
        .parse::<i32>()
        .unwrap();

    Pos { x, y }
}

fn parse_claw_machines(lines: &[String]) -> Vec<ClawMachine> {
    let mut claw_machines = Vec::new();
    let mut idx = 0;
    while idx < lines.len() {
        let a = parse_button(&lines[idx]);
        let b = parse_button(&lines[idx + 1]);
        let prize = parse_prize(&lines[idx + 2]);

        claw_machines.push(ClawMachine {
            button_a: a,
            button_b: b,
            prize,
        });
        idx += 4;
    }

    claw_machines
}

fn part1(claw_machines: &[ClawMachine]) -> usize {
    let mut result = 0;
    for claw_machine in claw_machines.iter() {
        let x_equation = Equation::<f32> {
            x: claw_machine.button_a.x as f32,
            y: claw_machine.button_b.x as f32,
            ans: claw_machine.prize.x as f32,
        };

        let y_equation = Equation::<f32> {
            x: claw_machine.button_a.y as f32,
            y: claw_machine.button_b.y as f32,
            ans: claw_machine.prize.y as f32,
        };

        let (x, y) = cramers_rule::<f32>(&x_equation, &y_equation);

        let allowed_range = 0.0..=100.0;
        if allowed_range.contains(&x)
            && allowed_range.contains(&y)
            && x.round() == x
            && y.round() == y
        {
            result += x as usize * 3 + y as usize;
        }
    }
    result
}
fn part2(claw_machines: &[ClawMachine]) -> usize {
    let mut result = 0;
    for claw_machine in claw_machines.iter() {
        let x_equation = Equation::<f64> {
            x: claw_machine.button_a.x as f64,
            y: claw_machine.button_b.x as f64,
            ans: claw_machine.prize.x as f64 + 10000000000000.0,
        };

        let y_equation = Equation::<f64> {
            x: claw_machine.button_a.y as f64,
            y: claw_machine.button_b.y as f64,
            ans: claw_machine.prize.y as f64 + 10000000000000.0,
        };

        let (x, y) = cramers_rule::<f64>(&x_equation, &y_equation);

        if x >= 0.0 && y >= 0.0 && x.round() == x && y.round() == y {
            result += x as usize * 3 + y as usize;
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_lines() -> Vec<ClawMachine> {
        let input = r#"Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279"#;
        let lines = parse_lines(&input);
        parse_claw_machines(&lines)
    }

    #[test]
    fn test_part1() -> Result<(), String> {
        let claw_machines = get_lines();
        assert_eq!(part1(&claw_machines), 480);

        Ok(())
    }

    #[test]
    fn test_part2() -> Result<(), String> {
        let claw_machines = get_lines();
        assert_eq!(part2(&claw_machines), 875318608908);

        Ok(())
    }
}
