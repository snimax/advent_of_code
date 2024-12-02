use advent_of_code_2024::{parse_file, parse_lines};

pub fn solve() {
  if let Ok(line_string) = parse_file("Inputs/day2.txt") {
    let lines = parse_lines(&line_string);
    println!("Part1 solution: {}", part1(&lines));
    println!("Part2 solution: {}", part2(&lines));
} else {
    println!("Could not parse file");
}
}

fn safe_levels(nums: &[i32]) -> bool
{
  let increasing = (nums[1] - nums[0]).is_positive();

  for (idx, curr) in (&nums[..(nums.len()-1)]).into_iter().enumerate()
  {
    let next: i32 = nums[idx + 1];
    let diff = next - curr;

    let safe_gap = (1..=3).contains(&diff.abs());
    let same_sign = if increasing { *curr < next } else { *curr > next };

    if !safe_gap || !same_sign
    {
      return false;
    }
  }

  true
}

fn part1(lines: &Vec<String>) -> i32
{
  let mut safe_levels = 0;
  for line in lines {
    let mut reports = Vec::new();
    let report_nums = line.split_ascii_whitespace();

    for report in report_nums.into_iter() {
      reports.push(report.parse::<i32>().unwrap());
    }

    let positive = reports.windows(2).next()
      .and_then(|list| Some((list[0]-list[1]).is_positive())).unwrap();

    let safe_report = reports.windows(2)
      .into_iter()
      .all(|list| {
        let diff = list[0] - list[1];
        (1..=3).contains(&diff.abs())
        && diff.is_positive() == positive
      });

    if safe_report {
      safe_levels += 1;
    }
  }

  safe_levels
}

fn part2(lines: &Vec<String>) -> i32
{
  let mut safe_level_count = 0;
  for line in lines {
    let mut reports = Vec::new();
    let report_nums = line.split_ascii_whitespace();

    for report in report_nums.into_iter() {
      reports.push(report.parse::<i32>().unwrap());
    }

    println!("Checking {reports:?}");
    if safe_levels(&reports)
    {
      println!("{reports:?}\n");
      safe_level_count += 1;
      continue;
    }

    for skip_index in 0..reports.len()
    {
      let subset: Vec<i32> = reports.iter()
        .enumerate()
        .filter(|(i, _)| *i != skip_index)
        .map(|(_, &n)| n)
        .collect();

      if safe_levels(&subset)
      {
        println!("{skip_index}: {subset:?}\n");
        safe_level_count += 1;
        break;
      }
    }
  }

  safe_level_count
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