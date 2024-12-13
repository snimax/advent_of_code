use advent_of_code_2024::{parse_file, parse_lines, Pos};
use std::collections::HashMap;

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

fn recurse(
    curr_pos: &Pos,
    a_uses_left: usize,
    b_uses_left: usize,
    claw_machine: &ClawMachine,
    memoization: &mut HashMap<(Pos, usize, usize), usize>,
) -> usize {
    if let Some(res) = memoization.get(&(curr_pos.clone(), a_uses_left, b_uses_left)) {
        return *res;
    }

    if curr_pos.x == 0 && curr_pos.y == 0 {
        memoization.insert((curr_pos.clone(), a_uses_left, b_uses_left), 0);
        return 0;
    }

    if a_uses_left == 0 && b_uses_left == 0 {
        let greater_than_max = 3 * 100 + 100 + 4; // Just more than maximum, but less than overflow 😅
        memoization.insert(
            (curr_pos.clone(), a_uses_left, b_uses_left),
            greater_than_max,
        );
        return greater_than_max;
    }

    if a_uses_left > 0 && b_uses_left > 0 {
        let used_a = 3 + recurse(
            &(curr_pos.clone() - claw_machine.button_a.clone()),
            a_uses_left - 1,
            b_uses_left,
            claw_machine,
            memoization,
        );
        let used_b = 1 + recurse(
            &(curr_pos.clone() - claw_machine.button_b.clone()),
            a_uses_left,
            b_uses_left - 1,
            claw_machine,
            memoization,
        );
        let result = usize::min(used_a, used_b);
        memoization.insert((curr_pos.clone(), a_uses_left, b_uses_left), result);
        return result;
    }

    if a_uses_left > 0 {
        let result = 3 + recurse(
            &(curr_pos.clone() - claw_machine.button_a.clone()),
            a_uses_left - 1,
            b_uses_left,
            claw_machine,
            memoization,
        );
        memoization.insert((curr_pos.clone(), a_uses_left, b_uses_left), result);
        return result;
    }

    // Only b uses left
    let result = 1 + recurse(
        &(curr_pos.clone() - claw_machine.button_b.clone()),
        a_uses_left,
        b_uses_left - 1,
        claw_machine,
        memoization,
    );
    memoization.insert((curr_pos.clone(), a_uses_left, b_uses_left), result);
    result
}

fn min_tokens_required(claw_machine: &ClawMachine) -> usize {
    let mut memoization = HashMap::new();
    let res = recurse(
        &claw_machine.prize,
        100,
        100,
        claw_machine,
        &mut memoization,
    );
    if res <= 3 * 100 + 100 {
        return res;
    }
    0
}

fn part1(claw_machines: &[ClawMachine]) -> usize {
    let mut result = 0;
    for claw_machine in claw_machines.iter() {
        result += min_tokens_required(claw_machine);
    }
    result
}
fn part2(claw_machines: &[ClawMachine]) -> usize {
    let mut result = 0;
    for claw_machine in claw_machines.iter() {
        // x1 x2 = x_ans
        // y1 y2 = y_ans
        let x1 = claw_machine.button_a.x as f64;
        let x2 = claw_machine.button_b.x as f64;
        let x_ans = claw_machine.prize.x as f64 + 10000000000000.0;

        let y1 = claw_machine.button_a.y as f64;
        let y2 = claw_machine.button_b.y as f64;
        let y_ans = claw_machine.prize.y as f64 + 10000000000000.0;

        // cramers rule
        let denominator = x1 * y2 - y1 * x2;

        let x_nominator = x_ans * y2 - y_ans * x2;
        let y_nominator = x1 * y_ans - y1 * x_ans;

        let x = x_nominator / denominator;
        let y = y_nominator / denominator;

        if x > 0.0 && y > 0.0 && x.round() == x && y.round() == y {
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
