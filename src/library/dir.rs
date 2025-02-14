use super::pos::*;
use std::ops::{Add, AddAssign, Deref, DerefMut, Mul, Sub, SubAssign};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Dir(pub Pos);

impl Dir {
    pub const fn new(x: i32, y: i32) -> Self {
        Dir(Pos { x, y })
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

impl AddAssign<&Dir> for Pos {
    fn add_assign(&mut self, other: &Dir) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
        };
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

impl SubAssign<&Dir> for Pos {
    fn sub_assign(&mut self, other: &Dir) {
        *self = Self {
            x: self.x - other.x,
            y: self.y - other.y,
        };
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
