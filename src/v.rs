use std::ops::{Add, Mul, Rem};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct V(pub i32, pub i32);


#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Facing {
    EAST,
    NORTH,
    WEST,
    SOUTH,
}

impl Facing {
    pub fn turn_left(&self) -> Self {
        match self {
            Facing::EAST => { Facing::NORTH }
            Facing::NORTH => { Facing::WEST }
            Facing::WEST => { Facing::SOUTH }
            Facing::SOUTH => { Facing::EAST }
        }
    }

    pub fn turn_right(&self) -> Self {
        match self {
            Facing::EAST => { Facing::SOUTH }
            Facing::NORTH => { Facing::EAST }
            Facing::WEST => { Facing::NORTH }
            Facing::SOUTH => { Facing::WEST }
        }
    }
}

impl Add for V {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        V(self.0 + rhs.0, self.1 + rhs.1)
    }
}
impl Add<Facing> for V {
    type Output = Self;

    fn add(self, rhs: Facing) -> Self::Output {
        match rhs {
            Facing::EAST => { V(self.0, self.1 + 1) }
            Facing::NORTH => { V(self.0 - 1, self.1) }
            Facing::WEST => { V(self.0, self.1 - 1) }
            Facing::SOUTH => { V(self.0 + 1, self.1) }
        }
    }
}

impl Mul<i32> for V {
    type Output = Self;

    fn mul(self, rhs: i32) -> Self::Output {
        V(self.0 * rhs, self.1 * rhs)
    }
}

impl Rem<Self> for V {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self::Output {
        V(self.0.rem_euclid(rhs.0), self.1.rem_euclid(rhs.1))
    }
}
