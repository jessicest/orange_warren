use std::{ops::Add, fmt::Display};

use num::Num;

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub struct Pos<T> {
    x: T,
    y: T,
}

impl <T: Num + Display> Pos<T> {
    pub fn new(x: T, y: T) -> Self {
        Pos {
            x,
            y,
        }
    }

    pub fn to_zone_id(&self) -> ZoneId {
        format!("z{},{}", self.x, self.y)
    }
}

impl <T: Num + Display> Add for Pos<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

pub struct Square<T> {
    nw: Pos<T>,
    width: T,
}

impl <T: Num + Display + Copy> Square<T> {
    pub fn center(&self) -> Pos<T> {
        self.nw + Pos::new(self.width, self.width)
    }
}

pub type ZoneId = String;

pub struct Zone {
    zid: ZoneId,
    area: Square<i64>,
    parents: Vec<Box<Zone>>,
    children: Vec<Box<Zone>>,
    neighbors: Vec<Box<Zone>>,
}

impl Zone {
    pub fn center(&self) -> Pos<i64> {
        self.area.center()
    }
}