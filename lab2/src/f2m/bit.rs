use std::ops::{Add, Sub, Mul, Div, BitOr, BitAnd, BitXor, Not, Deref};
use std::fmt::Display;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Bit {
    Zero,
    One
}

impl Display for Bit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self{
            Self::Zero => write!(f, "0"),
            Self::One => write!(f, "1"),
        }
    }
}

use Bit::*;

impl Not for Bit {
    type Output = Self;

    fn not(self) -> Self {
        match self {
            One => Zero,
            Zero => One,
        }
    }
}

impl BitOr for Bit {
    type Output = Self;

    fn bitor(self, other: Self) -> Self {
        match (self, other) {
            (Zero, Zero) => Zero,
            _ => One,
        }
    }
}

impl BitAnd for Bit {
    type Output = Bit;

    fn bitand(self, other: Self) -> Self {
        match (self, other) {
            (One, One) => One,
            _ => Zero,
        }
    }
}

impl BitXor for Bit {
    type Output = Bit;

    fn bitxor(self, other: Self) -> Self {
        match (self, other) {
            (One, One) => Zero,
            (Zero, Zero) => Zero,
            (Zero, One) => One,
            (One, Zero) => One,
        }
    }
}

impl Add for Bit {
    type Output = Bit;

    fn add(self, other: Self) -> Self {
        self ^ other
    }
}

impl Sub for Bit {
    type Output = Bit;

    fn sub(self, other: Self) -> Self {
        self ^ other
    }
}

impl Mul for Bit {
    type Output = Bit;

    fn mul(self, other: Self) -> Self {
        self & other
    }
}

impl Div for Bit {
    type Output = Bit;

    fn div(self, other: Self) -> Self {
        self
    }
}

impl Deref for Bit {
    type Target = bool;

    fn deref(&self) -> &bool {
        match self {
            Zero => &false,
            One => &true,
        }
    }
}