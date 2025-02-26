use std::collections::VecDeque;

use crate::years::AdventDay;
pub struct Day5 {}

impl AdventDay for Day5 {
    fn solve(&self) {
        let lines = self.get_input();
        let (seed_ranges, range_maps) = parse_input(&lines);
        println!("Part1 solution: {}", part1(&seed_ranges, &range_maps));
        println!("Part2 solution: {}", part2(&seed_ranges, &range_maps));
    }

    fn get_input_path(&self) -> &str {
        "Inputs/2023/day5.txt"
    }
}

#[derive(Debug, Clone)]
struct RangeMapping {
    length: u64,
    destination_start: u64,
    source_start: u64,
}

impl RangeMapping {
    fn destination_end(&self) -> u64 {
        self.destination_start + self.length - 1
    }

    fn source_end(&self) -> u64 {
        self.source_start + self.length - 1
    }

    fn contains(&self, value: u64) -> bool {
        self.source_start <= value && value <= self.source_end()
    }

    fn convert_val(&self, value: u64) -> u64 {
        if self.contains(value) {
            self.destination_start + (value - self.source_start)
        } else {
            value
        }
    }

    fn convert(&self, range: &SeedRange) -> (Option<SeedRange>, Vec<SeedRange>) {
        //       |$$$$$| mapping
        // |---|         range
        //
        // |---|         mapped interval
        if range.end < self.source_start {
            return (None, vec![range.clone()]);
        }
        // |$$$$$|       mapping
        //         |---| range
        //
        //         |---| mapped interval
        if range.start > self.source_end() {
            return (None, vec![range.clone()]);
        }

        // |$$$$$$$$| mapping
        //   |-----|  range
        //
        //   |$$$$$|  mapped_interval
        if self.contains(range.start) && self.contains(range.end) {
            let new_range = SeedRange {
                start: self.convert_val(range.start),
                end: self.convert_val(range.end),
            };
            return (Some(new_range), vec![]);
        }
        // |$$$$$$|      mapping
        //     |-------| range
        //
        // |$$$|--|      mapped_interval
        if self.contains(range.start) {
            let new_range = SeedRange {
                start: self.convert_val(range.start),
                end: self.destination_end(),
            };

            return (
                Some(new_range),
                if self.source_end() + 1 < range.end {
                    vec![SeedRange {
                        start: self.source_end() + 1,
                        end: range.end,
                    }]
                } else {
                    vec![]
                },
            );
        }
        //     |$$$$$$| mapping
        // |-------|    range
        //
        // |---|$$$|    mapped_interval
        if self.contains(range.end) {
            let new_range = SeedRange {
                start: self.destination_start,
                end: self.convert_val(range.end),
            };
            return (
                Some(new_range),
                if range.start < self.source_start - 1 {
                    vec![SeedRange {
                        start: range.start,
                        end: self.source_start - 1,
                    }]
                } else {
                    vec![]
                },
            );
        }

        //    |$$|   mapping
        // |-------| range
        //
        // |--|$$|-| mapped_interval

        let new_range = SeedRange {
            start: self.source_start,
            end: self.source_end(),
        };

        let mut remaining_ranges = Vec::new();

        if self.source_start > 0 {
            let remaining_start_range = SeedRange {
                start: range.start,
                end: self.source_start - 1,
            };
            if remaining_start_range.start <= remaining_start_range.end {
                remaining_ranges.push(remaining_start_range);
            }
        }

        let remaining_end_range = SeedRange {
            start: self.source_end() + 1,
            end: range.end,
        };
        if remaining_end_range.start <= remaining_end_range.end {
            remaining_ranges.push(remaining_end_range);
        }

        (Some(new_range), remaining_ranges)
    }
}

#[derive(Debug, Clone)]
struct SeedRange {
    start: u64,
    end: u64,
}

fn parse_input(lines: &[String]) -> (Vec<SeedRange>, Vec<Vec<RangeMapping>>) {
    let mut seed_ranges = Vec::new();
    let mut range_maps = Vec::new();

    let mut seed_ranges_iter = lines.iter();
    let range_maps_iter = lines.iter().skip(2);

    if let Some(line) = seed_ranges_iter.next() {
        let parts = line
            .split(':')
            .last()
            .expect("Expected seed ranges")
            .split_ascii_whitespace();
        for part in parts {
            let start = part.parse().expect("Expected a number");
            seed_ranges.push(SeedRange { start, end: start });
        }
    }

    let mut ranges = Vec::new();
    for line in range_maps_iter {
        if line.is_empty() {
            range_maps.push(ranges);
            ranges = Vec::new();
            continue;
        }
        if line.ends_with(':') {
            continue;
        }

        let mut parts = line.split_ascii_whitespace();
        let destination_start = parts
            .next()
            .expect("Expected a start of mapping destination")
            .parse()
            .expect("Expected a number");
        let source_start = parts
            .next()
            .expect("Expected an start of mapping source")
            .parse()
            .expect("Expected a number");
        let length = parts
            .next()
            .expect("Expected length of mapping table")
            .parse()
            .expect("Expected a number");

        ranges.push(RangeMapping {
            length,
            destination_start,
            source_start,
        });
    }

    if !ranges.is_empty() {
        range_maps.push(ranges);
    }

    (seed_ranges, range_maps)
}

fn map_ranges(seed_ranges: &[SeedRange], range_maps: &[RangeMapping]) -> Vec<SeedRange> {
    let mut sorted_ranges = seed_ranges.to_vec();
    sorted_ranges.sort_by_key(|range| range.start);
    let mut new_ranges = Vec::new();

    for seed_range in sorted_ranges {
        let old_len = new_ranges.len();
        let mut ranges_to_process = VecDeque::from(vec![seed_range.clone()]);
        while let Some(range) = ranges_to_process.pop_front() {
            let mut range_processed = false;
            for range_map in range_maps {
                let (mapped_range, remaining_ranges) = range_map.convert(&range);

                if let Some(mapped_range) = mapped_range {
                    new_ranges.push(mapped_range);
                    ranges_to_process.extend(remaining_ranges);
                    range_processed = true;
                    break;
                }
            }
            if !range_processed {
                new_ranges.push(range);
            }
        }
        if new_ranges.len() == old_len {
            new_ranges.push(seed_range.clone());
        }
    }

    new_ranges
}

fn part1(seed_ranges: &[SeedRange], range_maps: &[Vec<RangeMapping>]) -> u64 {
    let mut location_ranges = range_maps
        .iter()
        .fold(seed_ranges.to_vec(), |acc, range_map| {
            map_ranges(&acc, range_map)
        })
        .into_iter()
        .collect::<Vec<SeedRange>>();

    location_ranges.sort_by_key(|range| range.start);
    location_ranges.first().unwrap().start
}

fn convert_seed_range_inputs(seed_ranges: &[SeedRange]) -> Vec<SeedRange> {
    let mut new_ranges = Vec::new();

    for (start_range, end_range) in seed_ranges
        .iter()
        .zip(seed_ranges.iter().skip(1))
        .step_by(2)
    {
        new_ranges.push(SeedRange {
            start: start_range.start,
            end: start_range.start + end_range.start - 1, // Inclusive range
        });
    }
    new_ranges.sort_by_key(|range| range.start);
    new_ranges
}

fn part2(input_seed_ranges: &[SeedRange], range_maps: &[Vec<RangeMapping>]) -> u64 {
    let seed_ranges = convert_seed_range_inputs(input_seed_ranges);

    let mut range_maps = range_maps.to_vec();
    for range_map in range_maps.iter_mut() {
        range_map.sort_by_key(|range| range.source_start);
    }

    part1(&seed_ranges, &range_maps)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::library::*;

    fn get_lines() -> (Vec<SeedRange>, Vec<Vec<RangeMapping>>) {
        let input = r#"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4"#;

        parse_input(&parse_lines(&input))
    }

    #[test]
    fn test_part1() -> Result<(), String> {
        let (seed_ranges, range_maps) = get_lines();
        assert_eq!(part1(&seed_ranges, &range_maps), 35);

        Ok(())
    }

    #[test]
    fn test_part2() -> Result<(), String> {
        let (seed_ranges, range_maps) = get_lines();
        assert_eq!(part2(&seed_ranges, &range_maps), 46);

        Ok(())
    }
}
