use super::{parse_file, parse_lines};

pub fn solve() {
    if let Ok(line_string) = parse_file("Inputs/day25.txt") {
        let lines = parse_lines(&line_string);
        let (keys, locks) = parse_keys_and_locks(&lines);
        println!("Part1 solution: {}", part1(&keys, &locks));
    } else {
        println!("Could not parse file");
    }
}

type Key = [i32; 5];
type Lock = [i32; 5];

fn parse_keys_and_locks(lines: &[String]) -> (Vec<Key>, Vec<Lock>) {
    let mut keys = Vec::new();
    let mut locks = Vec::new();

    let mut key_or_lock = [0; 5];
    let mut parsing_key = false;
    let mut check_if_parsing_key = true;
    for line in lines {
        if line.is_empty() {
            // Remove the first or last line
            key_or_lock.iter_mut().for_each(|i| {
                *i -= 1;
            });

            if parsing_key {
                keys.push(key_or_lock);
            } else {
                locks.push(key_or_lock);
            }

            key_or_lock = [0; 5];
            check_if_parsing_key = true;
            continue;
        }
        if check_if_parsing_key {
            parsing_key = line.starts_with(".");
            check_if_parsing_key = false;
        }

        line.chars()
            .enumerate()
            .filter(|(_, c)| *c == '#')
            .for_each(|(idx, _)| {
                key_or_lock[idx] += 1;
            });
    }

    key_or_lock.iter_mut().for_each(|i| {
        *i -= 1;
    });

    if parsing_key {
        keys.push(key_or_lock);
    } else {
        locks.push(key_or_lock);
    }

    (keys, locks)
}

fn part1(keys: &[Key], locks: &[Lock]) -> usize {
    let mut keys_that_fit = 0;
    for lock in locks {
        for key in keys {
            if !lock.iter().zip(key).any(|(l, k)| l + k > 5) {
                keys_that_fit += 1;
            }
        }
    }

    keys_that_fit
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_input() -> (Vec<Key>, Vec<Lock>) {
        let input: &str = r#"#####
.####
.####
.####
.#.#.
.#...
.....

#####
##.##
.#.##
...##
...#.
...#.
.....

.....
#....
#....
#...#
#.#.#
#.###
#####

.....
.....
#.#..
###..
###.#
###.#
#####

.....
.....
.....
#....
#.#..
#.#.#
#####"#;

        let lines = parse_lines(&input);
        parse_keys_and_locks(&lines)
    }

    #[test]
    fn test_part1() -> Result<(), String> {
        let (keys, locks) = get_input();
        assert_eq!(part1(&keys, &locks), 3);

        Ok(())
    }
}
