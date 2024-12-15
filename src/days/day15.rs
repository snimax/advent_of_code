use advent_of_code_2024::{parse_file, parse_lines, Dir, Pos, DOWN, LEFT, RIGHT, UP};
use std::collections::HashSet;

pub fn solve() {
    if let Ok(line_string) = parse_file("Inputs/day15.txt") {
        let lines = parse_lines(&line_string);
        let (robot_pos, walls, boxes, instructions) = parse_map(&lines);
        println!(
            "Part1 solution: {}",
            part1(&robot_pos, &walls, &boxes, &instructions)
        );
        println!(
            "Part2 solution: {}",
            part2(&robot_pos, &walls, &boxes, &instructions)
        );
    } else {
        println!("Could not parse file");
    }
}

type Wall = Pos;
type Walls = HashSet<Wall>;
type Box = (usize, Pos);
type Boxes = Vec<Box>;

fn parse_map(lines: &[String]) -> (Pos, Walls, Boxes, Vec<Dir>) {
    let mut boxes = Boxes::new();
    let mut walls = Walls::new();
    let mut instructions = Vec::new();
    let mut robot_pos = Pos { x: 0, y: 0 };
    let mut idx = 0;

    let mut parse_instructuins = false;
    for (row, line) in lines.iter().enumerate() {
        if line.is_empty() {
            parse_instructuins = true;
            continue;
        }

        if !parse_instructuins {
            for (col, char) in line.chars().enumerate() {
                let x = col as i32;
                let y = row as i32;
                let pos = Pos { x, y };
                match char {
                    '#' => {
                        walls.insert(pos);
                    }
                    'O' => {
                        boxes.push((idx, pos));
                        idx += 1;
                    }
                    '@' => robot_pos = pos,
                    _ => {}
                }
            }
        } else {
            for char in line.chars() {
                instructions.push(match char {
                    '^' => UP,
                    'v' => DOWN,
                    '<' => LEFT,
                    '>' => RIGHT,
                    c => panic!("Gon unsupported instruction char {c} on line {row}"),
                })
            }
        }
    }

    (robot_pos, walls, boxes, instructions)
}

fn part1(start_robot_pos: &Pos, walls: &Walls, boxes: &Boxes, instructions: &[Dir]) -> usize {
    let mut curr_robot_pos = start_robot_pos.clone();
    let mut curr_box_positions = boxes.clone();

    for instruction in instructions {
        try_move_in_dir(
            &mut curr_robot_pos,
            &mut curr_box_positions,
            walls,
            instruction,
        );
    }

    curr_box_positions.iter().fold(0, |acc, (_, pos)| {
        acc + pos.x as usize + 100 * pos.y as usize
    })
}

fn find_next_free_spot(start_pos: &Pos, boxes: &Boxes, walls: &Walls, dir: &Dir) -> Option<Pos> {
    let mut curr_pos = start_pos.clone() + dir.clone();

    while boxes.iter().any(|(_, pos)| *pos == curr_pos) {
        curr_pos = curr_pos + dir.clone();
    }

    if walls.contains(&curr_pos) {
        return None;
    }

    Some(curr_pos)
}

fn move_affected_object(
    object: &mut Box,
    min_x: i32,
    max_x: i32,
    min_y: i32,
    max_y: i32,
    dir: &Dir,
) {
    let (_, ref mut pos) = object;
    if pos.x >= min_x && pos.x <= max_x && pos.y >= min_y && pos.y <= max_y {
        *pos = pos.clone() + dir.clone();
    }
}

fn try_move_in_dir(robot_pos: &mut Pos, boxes: &mut Boxes, walls: &Walls, dir: &Dir) {
    let new_robot_pos = robot_pos.clone() + dir.clone();

    if let Some(pos) = find_next_free_spot(robot_pos, boxes, walls, dir) {
        let min_x = i32::min(pos.x, robot_pos.x);
        let max_x = i32::max(pos.x, robot_pos.x);
        let min_y = i32::min(pos.y, robot_pos.y);
        let max_y = i32::max(pos.y, robot_pos.y);

        for box_obj in boxes.iter_mut() {
            move_affected_object(box_obj, min_x, max_x, min_y, max_y, dir);
        }
        *robot_pos = new_robot_pos;
    }
}

fn _print_map(robot_pos: &Pos, walls: &Walls, boxes: &Boxes, large: bool) {
    let mut max_x = 0;
    let mut max_y = 0;
    for pos in walls.iter() {
        max_x = i32::max(max_x, pos.x);
        max_y = i32::max(max_y, pos.y);
    }

    let mut box_count = 0;
    for y in 0..=max_y {
        let mut x = 0;
        loop {
            let curr_pos = Pos { x, y };

            if curr_pos == *robot_pos {
                print!("@");
            } else if walls.contains(&curr_pos) {
                print!("#");
            } else if let Some((idx, _)) = boxes.iter().find(|(_, pos)| *pos == curr_pos) {
                if large {
                    print!("{idx}");
                    if !boxes.iter().any(|(_, pos)| {
                        *pos == Pos {
                            x: curr_pos.x + 1,
                            y: curr_pos.y,
                        }
                    }) {
                        print!("]");
                        x += 1;
                    }
                } else {
                    print!("O")
                }
                box_count += 1;
            } else {
                print!(".");
            }
            x += 1;
            if x > max_x {
                break;
            }
        }
        println!();
    }
    println!("Boxes printed: {box_count}");
}

fn grow_map(start_pos: &Pos, boxes: &Boxes, walls: &Walls) -> (Pos, Boxes, Walls) {
    let new_start_pos = Pos {
        x: start_pos.x * 2,
        y: start_pos.y,
    };

    let mut new_boxes = Vec::new();
    for (idx, pos) in boxes.iter() {
        let new_pos = Pos {
            x: pos.x * 2,
            y: pos.y,
        };
        new_boxes.push((*idx, new_pos));
    }

    let mut new_walls = HashSet::new();
    for wall in walls {
        let new_pos = Pos {
            x: wall.x * 2,
            y: wall.y,
        };
        let new_pos1 = Pos {
            x: wall.x * 2 + 1,
            y: wall.y,
        };
        new_walls.insert(new_pos);
        new_walls.insert(new_pos1);
    }

    (new_start_pos, new_boxes, new_walls)
}

#[derive(PartialEq)]
enum Collision<'a> {
    None,
    Box(&'a Box),
    Wall,
}

fn find_overlapping_objects<'a>(robot_pos: &Pos, boxes: &'a Boxes, walls: &Walls) -> Collision<'a> {
    for b in boxes.iter() {
        if b.1 == *robot_pos || b.1.clone() + RIGHT == *robot_pos {
            return Collision::Box(b);
        }
    }
    if walls.contains(robot_pos) {
        return Collision::Wall;
    }
    Collision::None
}

fn items_overlapping(box1: &Box, box2: &Box) -> bool {
    let pos1_end = box1.1.clone() + RIGHT;
    let pos2_end = box2.1.clone() + RIGHT;

    pos1_end == box2.1 || pos2_end == box1.1 || box1.1 == box2.1
}

fn get_overlapping_boxes(boxes: &Boxes) -> Option<(&Box, &Box)> {
    for i in 0..boxes.len() {
        for j in i + 1..boxes.len() {
            if items_overlapping(&boxes[i], &boxes[j]) {
                return Some((&boxes[i], &boxes[j]));
            }
        }
    }
    None
}

fn move_boxes(dir: &Dir, walls: &Walls, moved_boxes: &[usize], boxes: &Boxes) -> Option<Boxes> {
    let mut new_boxes = boxes.clone();

    if let Some((box1, box2)) = get_overlapping_boxes(boxes) {
        let box1_moved_already = moved_boxes.contains(&box1.0);
        let box2_moved_already = moved_boxes.contains(&box2.0);
        if box1_moved_already && box2_moved_already {
            return None;
        }
        if !box1_moved_already {
            let mut new_moved_boxes = moved_boxes.to_owned();
            new_moved_boxes.push(box1.0);
            let new_pos = box1.1.clone() + dir.clone();
            let new_pos2 = new_pos.clone() + RIGHT;
            if walls.contains(&new_pos) || walls.contains(&new_pos2) {
                return None;
            }
            let b = new_boxes.iter_mut().find(|b| *b == box1).unwrap();
            b.1 = new_pos;
            let res = move_boxes(dir, walls, &new_moved_boxes, &new_boxes);
            if res.is_some() {
                return res;
            }
        }
        if !box2_moved_already {
            let new_pos = box2.1.clone() + dir.clone();
            let new_pos2 = new_pos.clone() + RIGHT;
            let mut new_moved_boxes = moved_boxes.to_owned();
            new_moved_boxes.push(box2.0);
            if walls.contains(&new_pos) || walls.contains(&new_pos2) {
                return None;
            }
            let b = new_boxes.iter_mut().find(|b| *b == box2).unwrap();
            b.1 = new_pos;
            let res = move_boxes(dir, walls, &new_moved_boxes, &new_boxes);
            if res.is_some() {
                return res;
            }
        }
        return None;
    }

    Some(new_boxes)
}

fn try_recursive_move_in_dir(robot_pos: &mut Pos, boxes: &mut Boxes, walls: &Walls, dir: &Dir) {
    let new_robot_pos = robot_pos.clone() + dir.clone();
    let overlapping_object = find_overlapping_objects(&new_robot_pos, boxes, walls);
    match overlapping_object {
        Collision::None => *robot_pos = new_robot_pos,
        Collision::Wall => {}
        Collision::Box(b) => {
            let mut new_boxes = boxes.clone();
            let new_obj_pos = b.1.clone() + dir.clone();
            let new_obj_pos2 = new_obj_pos.clone() + RIGHT;
            if walls.contains(&new_obj_pos) || walls.contains(&new_obj_pos2) {
                return;
            }
            if let Some(ref mut b) = new_boxes.iter_mut().find(|a| *a == b) {
                b.1 = new_obj_pos;
            }

            let mut moved_boxes = Vec::new();
            moved_boxes.push(b.0);
            if let Some(new_boxes) = move_boxes(dir, walls, &moved_boxes, &new_boxes) {
                *robot_pos = new_robot_pos;
                *boxes = new_boxes;
            }
        }
    }
}

fn part2(start_robot_pos: &Pos, walls: &Walls, boxes: &Boxes, instructions: &[Dir]) -> usize {
    let (mut curr_robot_pos, mut boxes, walls) = grow_map(start_robot_pos, boxes, walls);

    for instruction in instructions.iter() {
        try_recursive_move_in_dir(&mut curr_robot_pos, &mut boxes, &walls, instruction);
    }

    boxes.iter().fold(0, |acc, (_, pos)| {
        acc + pos.x as usize + 100 * pos.y as usize
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    fn get_smallest_input() -> (Pos, Walls, Boxes, Vec<Dir>) {
        let input = r#"#######
#...#.#
#.....#
#..OO@#
#..O..#
#.....#
#######

<vv<<^^<<^^"#;
        let lines = parse_lines(&input);
        parse_map(&lines)
    }

    fn get_small_input() -> (Pos, Walls, Boxes, Vec<Dir>) {
        let input = r#"########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<"#;

        let lines = parse_lines(&input);
        parse_map(&lines)
    }

    fn get_large_input() -> (Pos, Walls, Boxes, Vec<Dir>) {
        let input = r#"##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^"#;

        let lines = parse_lines(&input);
        parse_map(&lines)
    }

    #[test]
    fn test_part1_small() -> Result<(), String> {
        let (robot_pos, walls, boxes, instructions) = get_small_input();
        assert_eq!(part1(&robot_pos, &walls, &boxes, &instructions), 2028);

        Ok(())
    }

    #[test]
    fn test_part1_large() -> Result<(), String> {
        let (robot_pos, walls, boxes, instructions) = get_large_input();
        assert_eq!(part1(&robot_pos, &walls, &boxes, &instructions), 10092);

        Ok(())
    }

    #[test]
    fn test_part2_smallest() -> Result<(), String> {
        let (robot_pos, walls, boxes, instructions) = get_smallest_input();
        assert_eq!(part2(&robot_pos, &walls, &boxes, &instructions), 618);

        Ok(())
    }

    #[test]
    fn test_part2_large() -> Result<(), String> {
        let (robot_pos, walls, boxes, instructions) = get_large_input();
        assert_eq!(part2(&robot_pos, &walls, &boxes, &instructions), 9021);

        Ok(())
    }

    #[test]
    fn test_part2() -> Result<(), String> {
        let input = r#"##########
#........#
#..O.....#
#........#
#..OOO...#
#...O@...#
#........#
#........#
#........#
#........#
##########

<v<^^^^"#;

        let lines = parse_lines(&input);
        let (robot_pos, walls, boxes, instructions) = parse_map(&lines);

        assert_eq!(part2(&robot_pos, &walls, &boxes, &instructions), 1237);

        Ok(())
    }
}
