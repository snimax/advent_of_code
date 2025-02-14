use std::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign};

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

impl AddAssign<&Pos> for Pos {
    fn add_assign(&mut self, other: &Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
        };
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

impl SubAssign<&Pos> for Pos {
    fn sub_assign(&mut self, other: &Self) {
        *self = Self {
            x: self.x - other.x,
            y: self.y - other.y,
        };
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

impl MulAssign<i32> for Pos {
    fn mul_assign(&mut self, other: i32) {
        *self = Self {
            x: self.x * other,
            y: self.y * other,
        };
    }
}
