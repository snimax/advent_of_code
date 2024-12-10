use std::fs;
use std::io::Error;
use std::{ops::Add, ops::Sub};

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

pub type Dir = Pos;

pub const UP: Dir = Dir { x: 0, y: -1 };
pub const DOWN: Dir = Dir { x: 0, y: 1 };
pub const LEFT: Dir = Dir { x: -1, y: 0 };
pub const RIGHT: Dir = Dir { x: 1, y: 0 };

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
