use std::fs;
use std::io::Error;
use std::ops::{Add, Div, Mul, Sub, Deref, DerefMut};

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

impl<'a> Add<&'a Pos> for &Pos {
    type Output = Pos;

    fn add(self, other: &'a Pos) -> Pos {
        Pos {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl<'a> Sub<&'a Pos> for &Pos {
    type Output = Pos;

    fn sub(self, other: &'a Pos) -> Pos {
        Pos {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Mul<i32> for &Pos {
    type Output = Pos;

    fn mul(self, other: i32) -> Pos {
        Pos {
            x: self.x * other,
            y: self.y * other,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Dir(pub Pos);

impl Dir {
    pub const fn new(x: i32, y: i32) -> Self {
        Dir(Pos {x, y})
    }
}

impl Deref for Dir {
    type Target = Pos;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Dir {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<'a> Add<&'a Dir> for &Pos {
    type Output = Pos;

    fn add(self, other: &'a Dir) -> Pos {
        Pos {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl<'a> Sub<&'a Dir> for &Pos {
    type Output = Pos;

    fn sub(self, other: &'a Dir) -> Pos {
        Pos {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Mul<i32> for &Dir {
    type Output = Pos;

    fn mul(self, other: i32) -> Pos {
        Pos {
            x: self.x * other,
            y: self.y * other,
        }
    }
}

pub const UP: &Dir = &Dir(Pos { x: 0, y: -1 });
pub const DOWN: &Dir = &Dir(Pos { x: 0, y: 1 });
pub const LEFT: &Dir = &Dir(Pos { x: -1, y: 0 });
pub const RIGHT: &Dir = &Dir(Pos { x: 1, y: 0 });

pub const DIRECTIONS: [&Dir; 4] = [UP, DOWN, LEFT, RIGHT];

pub fn get_opposite_dir(dir: &Dir) -> &Dir {
    if dir == UP {
        return DOWN;
    }
    if dir == DOWN {
        return UP;
    }
    if dir == LEFT {
        return RIGHT;
    }
    if dir == RIGHT {
        return LEFT;
    }

    panic!("Got unknown direction {dir:?}");
}

pub fn get_dir_name(dir: &Dir) -> &str {
    if dir == UP {
        return "Down";
    }
    if dir == DOWN {
        return "Up";
    }
    if dir == LEFT {
        return "Right";
    }
    if dir == RIGHT {
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
