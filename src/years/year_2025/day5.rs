use crate::years::AdventDay;
use std::ops::RangeInclusive;

pub struct Day5 {}

impl AdventDay for Day5 {
    fn solve(&self) {
        let lines = self.get_input();
        println!("Part1 solution: {}", part1(&lines));
        println!("Part2 solution: {}", part2(&lines));
    }

    fn get_input_path(&self) -> &str {
        "Inputs/2025/day5.txt"
    }
}

type IngredientId = u64;
type FreshIngredientIds = RangeInclusive<IngredientId>;

fn parse_input(lines: &[String]) -> (Vec<FreshIngredientIds>, Vec<IngredientId>) {
    let mut ingredient_ids = Vec::new();
    let mut fresh_ingredient_ids = Vec::new();

    let mut fresh_ingredients_parsed = false;
    for line in lines {
        if line.is_empty() {
            fresh_ingredients_parsed = true;
            continue;
        }

        if fresh_ingredients_parsed {
            if let Ok(ingredient_id) = line.parse() {
                ingredient_ids.push(ingredient_id)
            }
        } else if let Some((start_str, end_str)) = line.split_once('-')
            && let Ok(start) = start_str.parse()
            && let Ok(end) = end_str.parse()
        {
            fresh_ingredient_ids.push(FreshIngredientIds::new(start, end));
        }
    }

    (fresh_ingredient_ids, ingredient_ids)
}

fn find_fresh_ingredient_ids(
    fresh_ingredient_ids: &[FreshIngredientIds],
    ingredient_ids: &[IngredientId],
) -> Vec<IngredientId> {
    let mut fresh_ones = Vec::new();
    for ingredient_id in ingredient_ids {
        if fresh_ingredient_ids
            .iter()
            .any(|fresh_range| fresh_range.contains(ingredient_id))
        {
            fresh_ones.push(*ingredient_id);
        }
    }
    fresh_ones
}

fn part1(lines: &[String]) -> usize {
    let (fresh_ingredient_ids, ingredient_ids) = parse_input(lines);
    find_fresh_ingredient_ids(&fresh_ingredient_ids, &ingredient_ids)
        .iter()
        .len()
}

fn sort_ranges(ranges: &mut [FreshIngredientIds]) {
    ranges.sort_by(|a, b| a.start().cmp(b.start()));
}

fn merge_ranges(ranges: &[FreshIngredientIds]) -> Vec<FreshIngredientIds> {
    let mut ranges = ranges.to_vec();
    sort_ranges(&mut ranges);
    let mut new_ranges = ranges.to_vec();

    for range1_idx in 0..ranges.len() {
        for range2_idx in range1_idx + 1..ranges.len() {
            let range1 = &ranges[range1_idx];
            let range2 = &ranges[range2_idx];

            if range1.contains(range2.start()) {
                if range1.contains(range2.end()) {
                    new_ranges.remove(range2_idx);
                } else {
                    new_ranges.push(RangeInclusive::new(*range1.start(), *range2.end()));
                    new_ranges.remove(range2_idx); // Remove the largest index first
                    new_ranges.remove(range1_idx);
                }
            } else if range2.contains(range1.start()) {
                if range2.contains(range1.end()) {
                    new_ranges.remove(range1_idx);
                } else {
                    new_ranges.push(RangeInclusive::new(*range1.start(), *range2.end()));
                    new_ranges.remove(range2_idx); // Remove the largest index first
                    new_ranges.remove(range1_idx);
                }
            }

            if new_ranges.len() != ranges.len() {
                new_ranges = merge_ranges(&new_ranges);
                return new_ranges;
            }
        }
    }
    new_ranges
}

fn part2(lines: &[String]) -> u64 {
    let (mut fresh_ingredient_ids, _ingredient_ids) = parse_input(lines);
    fresh_ingredient_ids = merge_ranges(&fresh_ingredient_ids);

    let mut fresh_ids = 0;
    for range in fresh_ingredient_ids {
        fresh_ids += range.end() - range.start() + 1;
    }
    fresh_ids
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::library::*;

    fn get_lines() -> Vec<String> {
        let input = r#"3-5
10-14
16-20
12-18

1
5
8
11
17
32"#;

        parse_lines(&input)
    }

    #[test]
    fn test_part1() -> Result<(), String> {
        let lines = get_lines();
        assert_eq!(part1(&lines), 3);

        Ok(())
    }

    #[test]
    fn test_part2() -> Result<(), String> {
        let lines = get_lines();
        assert_eq!(part2(&lines), 14);

        Ok(())
    }
}
