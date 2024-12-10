use advent_of_code_2024::parse_file;

pub fn solve() {
    if let Ok(line_string) = parse_file("Inputs/day9.txt") {
        let disk = parse_disk_string(&line_string);
        println!("Part1 solution: {}", part1(&disk));
        println!("Part2 solution: {}", part2(&line_string));
    } else {
        println!("Could not parse file");
    }
}

fn parse_disk_string(str: &str) -> Vec<Option<u32>> {
    let parsed_disk_representation = str
        .chars()
        .map(|c| c.to_string().parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    let mut disk_representation = Vec::new();
    let mut file_id = 0;
    for (idx, val) in parsed_disk_representation.iter().enumerate() {
        let block = if idx % 2 == 0 { Some(file_id) } else { None };
        disk_representation.append(&mut vec![block; *val as usize]);

        if idx % 2 == 0 {
            file_id += 1
        }
    }
    disk_representation
}

fn part1(disk_representation: &[Option<u32>]) -> usize {
    let mut idx = 0;
    let mut rev_idx = disk_representation.len() - 1;

    let mut defragmented_disk = Vec::new();

    while idx <= rev_idx {
        match disk_representation[idx] {
            Some(val) => defragmented_disk.push(val),
            None => {
                while disk_representation[rev_idx].is_none() && rev_idx > idx {
                    rev_idx -= 1;
                }
                if disk_representation[rev_idx].is_some() {
                    defragmented_disk.push(disk_representation[rev_idx].unwrap());
                    rev_idx -= 1;
                }
            }
        }
        idx += 1;
    }

    defragmented_disk
        .iter()
        .enumerate()
        .fold(0, |acc, (idx, &val)| acc + idx * val as usize)
}

type Id = usize;

#[derive(Debug, Clone, PartialEq)]
enum Block {
    File(Id, usize),
    Empty(usize),
}

fn merge_empty_spaces(disk: &mut Vec<Block>) {
    let mut idx = 0;
    while idx < disk.len() - 1 {
        if let Block::Empty(size) = disk[idx] {
            if size == 0 {
                disk.remove(idx);
                continue;
            }
            if let Block::Empty(size2) = disk[idx + 1] {
                disk.remove(idx);
                disk.remove(idx);
                disk.insert(idx, Block::Empty(size + size2));
                continue;
            }
        }
        idx += 1;
    }
}

fn part2(str: &str) -> usize {
    let parsed_disk_representation = str
        .chars()
        .map(|c| c.to_string().parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    let mut disk_representation = Vec::new();
    let mut file_id = 0;
    for (idx, val) in parsed_disk_representation.iter().enumerate() {
        if idx % 2 == 0 {
            disk_representation.push(Block::File(file_id, *val));
        } else if *val > 0 {
            disk_representation.push(Block::Empty(*val));
        };

        if idx % 2 == 0 {
            file_id += 1
        }
    }

    let mut rev_idx = disk_representation.len() - 1;
    let mut defragmented_disk = disk_representation.clone();

    while rev_idx > 0 {
        if let Block::File(id, file_size) = disk_representation[rev_idx] {
            for (idx, block) in defragmented_disk.iter().enumerate() {
                match block {
                    Block::File(_, _) => {
                        continue;
                    }
                    Block::Empty(empty_space) => {
                        if *empty_space >= file_size {
                            let moved_file_idx = defragmented_disk
                                .iter()
                                .position(|block| {
                                    if let Block::File(id2, _) = block {
                                        *id2 == id
                                    } else {
                                        false
                                    }
                                })
                                .unwrap();

                            if idx > moved_file_idx {
                                continue;
                            }

                            let space_left = empty_space - file_size;
                            let mut new_disk = defragmented_disk.clone();
                            new_disk[idx] = Block::File(id, file_size);
                            new_disk[moved_file_idx] = Block::Empty(file_size);
                            new_disk.insert(idx + 1, Block::Empty(space_left));
                            merge_empty_spaces(&mut new_disk);

                            defragmented_disk = new_disk;
                            break;
                        }
                    }
                }
            }
        }
        rev_idx -= 1;
    }

    merge_empty_spaces(&mut defragmented_disk);

    let mut result = 0;

    let mut idx = 0;
    for block in defragmented_disk.iter() {
        match block {
            Block::File(id, size) => {
                for _ in 0..*size {
                    result += id * idx;
                    idx += 1;
                }
            }
            Block::Empty(size) => idx += size,
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_line() -> String {
        r#"2333133121414131402"#.to_string()
    }

    #[test]
    fn test_part1() -> Result<(), String> {
        let disk = parse_disk_string(&get_line());
        assert_eq!(part1(&disk), 1928);

        Ok(())
    }

    #[test]
    fn test_part2() -> Result<(), String> {
        assert_eq!(part2(&get_line()), 2858);

        Ok(())
    }
}
