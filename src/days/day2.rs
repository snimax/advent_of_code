use super::{parse_file, parse_lines};

pub fn solve() {
    if let Ok(line_string) = parse_file("Inputs/day2.txt") {
        let lines = parse_lines(&line_string);
        println!("Part1 solution: {}", part1(&lines));
        println!("Part2 solution: {}", part2(&lines));
    } else {
        println!("Could not parse file");
    }
}

fn safe_levels(nums: &[i32]) -> bool {
    let increasing = (nums[1] - nums[0]).is_positive();

    for (idx, curr) in (nums[..(nums.len() - 1)]).iter().enumerate() {
        let next: i32 = nums[idx + 1];
        let diff = next - curr;

        let safe_gap = (1..=3).contains(&diff.abs());
        let same_sign = if increasing {
            *curr < next
        } else {
            *curr > next
        };

        if !safe_gap || !same_sign {
            return false;
        }
    }

    true
}

fn part1(lines: &Vec<String>) -> i32 {
    let mut num_safe_reports = 0;
    for line in lines {
        let mut reports = Vec::new();
        let report_nums = line.split_ascii_whitespace();

        for report in report_nums.into_iter() {
            reports.push(report.parse::<i32>().unwrap());
        }

        if safe_levels(&reports) {
            num_safe_reports += 1;
        }
    }
    num_safe_reports
}

fn part2(lines: &Vec<String>) -> i32 {
    let mut num_safe_reports = 0;
    for line in lines {
        let mut reports = Vec::new();
        let report_nums = line.split_ascii_whitespace();

        for report in report_nums.into_iter() {
            reports.push(report.parse::<i32>().unwrap());
        }

        println!("Checking {reports:?}");
        if safe_levels(&reports) {
            println!("{reports:?}\n");
            num_safe_reports += 1;
            continue;
        }

        for skip_index in 0..reports.len() {
            let subset: Vec<i32> = reports
                .iter()
                .enumerate()
                .filter(|(i, _)| *i != skip_index)
                .map(|(_, &n)| n)
                .collect();

            if safe_levels(&subset) {
                println!("{skip_index}: {subset:?}\n");
                num_safe_reports += 1;
                break;
            }
        }
    }

    num_safe_reports
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_lines() -> Vec<String> {
        let input = r#"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"#;

        parse_lines(&input)
    }

    #[test]
    fn test_part1() -> Result<(), String> {
        let lines = get_lines();
        assert_eq!(part1(&lines), 2);

        Ok(())
    }

    #[test]
    fn test_part2() -> Result<(), String> {
        let lines = get_lines();
        assert_eq!(part2(&lines), 4);

        Ok(())
    }
}
