use fixed::types::{I20F12, U20F12};

pub type FInt = I20F12;
pub type FUint = U20F12;

pub struct Coordinate2d {
    pub x: FInt,
    pub y: FInt,
}

impl Coordinate2d {
    pub fn new(x: FInt, y: FInt) -> Self {
        return Self { x: x, y: y };
    }

    pub fn zero() -> Self {
        let zero = FInt::from_num(0);
        return Self::new(zero, zero);
    }

    pub fn one() -> Self {
        let one = FInt::from_num(1);
        return Self::new(one, one);
    }
}
