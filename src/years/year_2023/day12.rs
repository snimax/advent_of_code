use std::collections::HashMap;

use crate::years::AdventDay;

pub struct Day12 {}

impl AdventDay for Day12 {
    fn solve(&self) {
        let lines = self.get_input();
        let spring_statuses = parse_records(&lines);
        println!("Part1 solution: {}", part1(&spring_statuses));
        println!("Part2 solution: {}", part2(&spring_statuses));
    }

    fn get_input_path(&self) -> &str {
        "Inputs/2023/day12.txt"
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Status {
    Operational,
    Damage,
    Unknown,
}

type SpringStatus = (Vec<Status>, Vec<usize>);

fn compare_slices(slice: &[Status], other: &[Status]) -> bool {
    slice
        .iter()
        .zip(other)
        .all(|(s, o)| s == o || *o == Status::Unknown)
}

fn match_substr(
    pos: usize,
    substr_idx: usize,
    str_status: &[Status],
    sub_str_statuses: &[Vec<Status>],
    memoization: &mut HashMap<(usize, usize), usize>,
) -> usize {
    if let Some(res) = memoization.get(&(pos, substr_idx)) {
        return *res;
    }

    if substr_idx >= sub_str_statuses.len() {
        if str_status
            .iter()
            .skip(pos)
            .any(|status| *status == Status::Damage)
        {
            memoization.insert((pos, substr_idx), 0);
            return 0;
        }
        memoization.insert((pos, substr_idx), 1);
        return 1; // We have matched all groups, thus this is a permutation that works. Do we need to test futher?
    }

    let substr = &sub_str_statuses[substr_idx];
    if pos + substr.len() > str_status.len() {
        memoization.insert((pos, substr_idx), 0);
        return 0; // We are out of characters in the string we're supposed to match. No need to test the other positions
    }

    let mut permutations = 0;
    if compare_slices(substr, &str_status[pos..pos + substr.len()]) {
        let new_pos = pos + substr.len() - 1;
        let new_substr_idx = substr_idx + 1;
        let val = match_substr(
            new_pos,
            new_substr_idx,
            str_status,
            sub_str_statuses,
            memoization,
        );
        memoization.insert((new_pos, new_substr_idx), val);
        permutations += val;
    }

    if str_status[pos] != Status::Damage {
        let new_pos = pos + 1;
        let val = match_substr(
            new_pos,
            substr_idx,
            str_status,
            sub_str_statuses,
            memoization,
        );
        memoization.insert((new_pos, substr_idx), val);
        permutations += val;
    }

    permutations
}

fn part1(spring_statuses: &[SpringStatus]) -> usize {
    let mut appended_spring_statuses = spring_statuses.to_owned();

    // Appending operational status to both ends to avoid edge cases :D
    for (str_statuses, _) in appended_spring_statuses.iter_mut() {
        str_statuses.insert(0, Status::Operational);
        str_statuses.push(Status::Operational);
    }

    appended_spring_statuses
        .iter()
        .map(|(str_status, int_status)| {
            let int_groups_as_str_status = int_status
                .iter()
                .map(|num| {
                    let mut group = vec![Status::Operational];
                    group.append(&mut vec![Status::Damage; *num]);
                    group.push(Status::Operational);
                    group
                })
                .collect::<Vec<Vec<Status>>>();

            let mut memoization = HashMap::new();
            match_substr(
                0,
                0,
                str_status,
                &int_groups_as_str_status,
                &mut memoization,
            )
        })
        .sum()
}

fn part2(spring_statuses: &[SpringStatus]) -> usize {
    let mut unfolded_spring_statuses = Vec::with_capacity(spring_statuses.len());

    for (str_status, int_status) in spring_statuses {
        let mut unfolded_str_status = str_status.clone();
        let mut unfolded_int_status = int_status.clone();
        for _ in 0..4 {
            unfolded_str_status.push(Status::Unknown);
            unfolded_str_status.append(&mut str_status.clone());
            unfolded_int_status.append(&mut int_status.clone());
        }
        unfolded_spring_statuses.push((unfolded_str_status, unfolded_int_status));
    }
    part1(&unfolded_spring_statuses)
}

fn parse_string_status(line: &str) -> Vec<Status> {
    line.chars()
        .map(|c| match c {
            '.' => Status::Operational,
            '#' => Status::Damage,
            '?' => Status::Unknown,
            _ => panic!("Got unexpected char '{c}' when trying to parse string status"),
        })
        .collect()
}

fn parse_integer_status(line: &str) -> Vec<usize> {
    line.split(',').filter_map(|i| i.parse().ok()).collect()
}

fn parse_records(lines: &[String]) -> Vec<SpringStatus> {
    lines
        .iter()
        .map(|line| {
            let mut parts = line.split_ascii_whitespace();
            let str_part = parts
                .next()
                .expect("Status as a line consisting of '.', '#' and '?'");
            let int_part = parts
                .next()
                .expect("Status on the form of comma separated integers");

            (
                parse_string_status(str_part),
                parse_integer_status(int_part),
            )
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::library::*;

    #[test]
    fn test_part1() -> Result<(), String> {
        let spring_statuses = get_input();
        assert_eq!(part1(&spring_statuses), 21);

        Ok(())
    }

    #[test]
    fn test_part2() -> Result<(), String> {
        let spring_statuses = get_input();
        assert_eq!(part2(&spring_statuses), 525152);

        Ok(())
    }

    fn get_input<'a>() -> Vec<SpringStatus> {
        let input = r#"???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1"#;

        parse_records(&parse_lines(&input))
    }
}
