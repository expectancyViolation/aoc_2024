use std::fmt::{Display, Formatter};

pub(crate) struct StrMap<'a> {
    pub(crate) data: &'a mut [u8],
    pub(crate) h: i32,
    pub(crate) w: i32,
}

impl<'a> StrMap<'a> {
    pub(crate) fn get(&self, x: i32, y: i32) -> u8 {
        if x < 0 || y < 0 || (self.h <= x) || (self.w <= y) {
            255
        } else { self.data[(x * (self.w + 1) + y) as usize] }
    }
    pub(crate) fn set(&mut self, x: i32, y: i32, val: u8) {
        self.data[(x * (self.w + 1) + y) as usize] = val;
    }

    pub(crate) fn find(&self, val: u8) -> Option<(i32, i32)> {
        self.data.iter().position(|&x| x == val).map(
            |pos| {
                let x = (pos as i32) / (self.w + 1);
                let y = (pos as i32) % (self.w + 1);
                (x, y)
            }
        )
    }
}

impl<'a> Display for StrMap<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", std::str::from_utf8(self.data).unwrap())
    }
}

pub(crate) const DIRECTIONS: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

