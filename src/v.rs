use std::ops::{Add, Mul, Rem};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct V(pub i32, pub i32);

impl Add for V {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        V(self.0 + rhs.0, self.1 + rhs.1)
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
