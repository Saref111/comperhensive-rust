use std::{fmt::Display, ops::Mul};

#[allow(non_camel_case_types)]
pub struct bool(core::primitive::bool);

impl From<i16> for bool {
    fn from(x: i16) -> Self {
        Self(x > 0)
    }
}

impl Into<i16> for bool {
    fn into(self) -> i16 {
        if self.0 {
            1
        } else {
            0
        }
    }
}

impl From<i8> for bool {
    fn from(x: i8) -> Self {
        Self(x > 0)
    }
}

impl Into<i8> for bool {
    fn into(self) -> i8 {
        if self.0 {
            1
        } else {
            0
        }
    }
}

impl Mul<bool> for bool {
    type Output = usize;

    fn mul(self, rhs: Self) -> Self::Output {
        if self.0 && rhs.0 {
            1
        } else {
            0
        }
    }
}

impl Display for bool {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}


fn multiply_bool(x: bool, y: bool) -> usize {
    x * y
}

fn main() {
    let x: i8 = -15;
    let y: i16 = 1000;

    println!("{x} * {y} = {}", multiply_bool(x.into(), y.into()));
}

