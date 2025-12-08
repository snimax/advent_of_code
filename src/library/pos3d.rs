use std::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Pos3d {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl<'a> Add<&'a Pos3d> for &Pos3d {
    type Output = Pos3d;

    fn add(self, other: &'a Pos3d) -> Pos3d {
        Pos3d {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl AddAssign<&Pos3d> for Pos3d {
    fn add_assign(&mut self, other: &Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        };
    }
}

impl<'a> Sub<&'a Pos3d> for &Pos3d {
    type Output = Pos3d;

    fn sub(self, other: &'a Pos3d) -> Pos3d {
        Pos3d {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl SubAssign<&Pos3d> for Pos3d {
    fn sub_assign(&mut self, other: &Self) {
        *self = Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        };
    }
}

impl Mul<i32> for &Pos3d {
    type Output = Pos3d;

    fn mul(self, other: i32) -> Pos3d {
        Pos3d {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}

impl MulAssign<i32> for Pos3d {
    fn mul_assign(&mut self, other: i32) {
        *self = Self {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        };
    }
}
