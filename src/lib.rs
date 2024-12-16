use std::fs;
use std::io::Error;
use std::ops::{Add, Div, Mul, Sub};

pub fn parse_file(file: &str) -> Result<String, Error> {
    fs::read_to_string(file)
}

pub fn parse_lines(s: &str) -> Vec<String> {
    let mut lines: Vec<String> = Vec::new();

    for line in s.lines() {
        lines.push(line.to_string());
    }

    lines
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Pos {
    pub x: i32,
    pub y: i32,
}

impl Add for Pos {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for Pos {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Mul<i32> for Pos {
    type Output = Self;

    fn mul(self, rhs: i32) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

pub type Dir = Pos;

pub const UP: Dir = Dir { x: 0, y: -1 };
pub const DOWN: Dir = Dir { x: 0, y: 1 };
pub const LEFT: Dir = Dir { x: -1, y: 0 };
pub const RIGHT: Dir = Dir { x: 1, y: 0 };

pub const DIRECTIONS: [Dir; 4] = [UP, DOWN, LEFT, RIGHT];

pub fn get_opposite_dir(dir: &Dir) -> Dir {
    if *dir == UP {
        return DOWN;
    }
    if *dir == DOWN {
        return UP;
    }
    if *dir == LEFT {
        return RIGHT;
    }
    if *dir == RIGHT {
        return LEFT;
    }

    panic!("Got unknown direction {dir:?}");
}

pub fn get_dir_name(dir: &Dir) -> &str {
    if *dir == UP {
        return "Down";
    }
    if *dir == DOWN {
        return "Up";
    }
    if *dir == LEFT {
        return "Right";
    }
    if *dir == RIGHT {
        return "Left";
    }

    panic!("Got unknown direction {dir:?}");
}

#[derive(Clone)]
pub struct Map<T: Copy> {
    pub map: Vec<Vec<T>>,
    pub size_x: usize,
    pub size_y: usize,
}

impl<T: Copy> Map<T> {
    pub fn next(&self, curr_pos: &Pos, dir: &Dir) -> Option<T> {
        let new_pos = Pos {
            x: curr_pos.x + dir.x,
            y: curr_pos.y + dir.y,
        };
        match self.valid_pos(&new_pos) {
            false => None,
            true => Some(self.get(&new_pos)),
        }
    }

    pub fn get(&self, pos: &Pos) -> T {
        self.map[pos.y as usize][pos.x as usize]
    }

    pub fn set(&mut self, pos: &Pos, val: T) {
        self.map[pos.y as usize][pos.x as usize] = val;
    }

    pub fn valid_pos(&self, pos: &Pos) -> bool {
        if pos.x < 0 || pos.y < 0 || pos.x >= self.size_x as i32 || pos.y >= self.size_y as i32 {
            return false;
        }
        true
    }
}

pub trait NumericOps:
    Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
    + PartialEq
    + PartialOrd
    + Copy
{
}
impl<T> NumericOps for T where
    T: Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + Div<Output = T>
        + PartialEq
        + PartialOrd
        + Copy
{
}

#[derive(PartialEq, Clone, Debug)]
pub struct Equation<T: NumericOps> {
    pub x: T,
    pub y: T,
    pub ans: T,
}

pub fn cramers_rule<T: NumericOps>(eq1: &Equation<T>, eq2: &Equation<T>) -> (T, T) {
    let denominator = eq1.x * eq2.y - eq2.x * eq1.y;
    let x_nominator = eq1.ans * eq2.y - eq1.y * eq2.ans;
    let y_nominator = eq1.x * eq2.ans - eq1.ans * eq2.x;

    (x_nominator / denominator, y_nominator / denominator)
}
