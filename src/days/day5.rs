use advent_of_code_2024::{parse_file, parse_lines};
use std::{cmp::Ordering, collections::HashMap};

pub fn solve() {
    if let Ok(line_string) = parse_file("Inputs/day5.txt") {
        let lines = parse_lines(&line_string);
        let input = parse_input(&lines);
        println!("Part1 solution: {}", part1(&input.0, &input.1));
        println!("Part2 solution: {}", part2(&input.0, &input.1));
    } else {
        println!("Could not parse file");
    }
}

fn parse_page_rules(line: &str) -> (u32, u32) {
    let pages: Vec<&str> = line.split('|').collect();
    // Assuming only two pages
    let first_page = pages[0].parse::<u32>().unwrap();
    let second_page = pages[1].parse::<u32>().unwrap();
    (first_page, second_page)
}

fn parse_pages_to_produce(line: &str) -> Vec<u32> {
    line.split(',')
        .collect::<Vec<&str>>()
        .iter()
        .map(|v| v.parse::<u32>().unwrap())
        .collect()
}

fn parse_input(lines: &[String]) -> (HashMap<u32, Vec<u32>>, Vec<Vec<u32>>) {
    let mut page_ordering_rules: HashMap<u32, Vec<u32>> = HashMap::new();
    let mut pages_to_produce: Vec<Vec<u32>> = vec![];

    let mut parsing_rules = true;
    for line in lines {
        if line.is_empty() {
            parsing_rules = false;
            continue;
        }

        if parsing_rules {
            let (a, b) = parse_page_rules(line);
            if let Some(val) = page_ordering_rules.get_mut(&a) {
                val.push(b);
            } else {
                page_ordering_rules.insert(a, vec![b]);
            }
        } else {
            pages_to_produce.push(parse_pages_to_produce(line));
        }
    }

    (page_ordering_rules, pages_to_produce)
}

fn check_correct_pages(pages: &[u32], rules: &HashMap<u32, Vec<u32>>) -> usize {
    for (page_num, page) in pages.iter().enumerate().rev() {
        if let Some(rule) = rules.get(page) {
            if pages[0..page_num].iter().any(|val| rule.contains(val)) {
                return 0;
            }
        }
    }
    pages[pages.len() / 2] as usize
}

fn part1(page_ordering_rules: &HashMap<u32, Vec<u32>>, pages_to_produce: &Vec<Vec<u32>>) -> usize {
    let mut result = 0;
    for pages in pages_to_produce {
        result += check_correct_pages(pages, page_ordering_rules);
    }

    result
}

fn fix_page(pages: &[u32], rules: &HashMap<u32, Vec<u32>>) -> usize {
    let mut sorted_pages = pages.to_vec();

    // Sorts the list in the reverse order, but solves the problem
    sorted_pages.sort_by(|a, b| {
        if let Some(values) = rules.get(b) {
            if values.contains(a) {
                return Ordering::Less;
            }
        }
        Ordering::Equal
    });
    sorted_pages[sorted_pages.len() / 2] as usize
}

fn part2(page_ordering_rules: &HashMap<u32, Vec<u32>>, pages_to_produce: &Vec<Vec<u32>>) -> usize {
    let mut result = 0;
    for pages in pages_to_produce {
        if check_correct_pages(pages, page_ordering_rules) == 0 {
            result += fix_page(pages, page_ordering_rules);
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_lines() -> (HashMap<u32, Vec<u32>>, Vec<Vec<u32>>) {
        let input = r#"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47"#;

        parse_input(&parse_lines(&input))
    }

    #[test]
    fn test_part1() -> Result<(), String> {
        let input = get_lines();
        assert_eq!(part1(&input.0, &input.1), 143);

        Ok(())
    }

    #[test]
    fn test_part2() -> Result<(), String> {
        let input = get_lines();
        assert_eq!(part2(&input.0, &input.1), 123);

        Ok(())
    }
}
