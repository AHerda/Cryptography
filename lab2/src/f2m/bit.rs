use std::fmt::Display;
use std::ops::{Add, BitAnd, BitOr, BitXor, Div, Mul, Neg, Not, Sub};

use crate::traits::{Field, Pow};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Bits8(pub u8);

impl Field for Bits8 {}

impl Display for Bits8 {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(fmt, "{:b}", self.0)
    }
}

impl Pow for Bits8 {
    fn one(&self) -> Self {
        Self(1)
    }

    fn zero(&self) -> Self {
        Self(0)
    }

    fn pow(self, exponent: usize) -> Self {
        if self == self.zero() && exponent == 0 {
            panic!("Zero raised to the power of zero is undefined");
        }
        self
    }
}

impl Not for Bits8 {
    type Output = Self;

    fn not(self) -> Self {
        Self(!self.0)
    }
}

impl BitOr for Bits8 {
    type Output = Self;

    fn bitor(self, other: Self) -> Self::Output {
        Self(self.0 | other.0)
    }
}

impl BitAnd for Bits8 {
    type Output = Bits8;

    fn bitand(self, other: Self) -> Self::Output {
        Self(self.0 & other.0)
    }
}

impl BitXor for Bits8 {
    type Output = Bits8;

    fn bitxor(self, other: Self) -> Self::Output {
        Self(self.0 ^ other.0)
    }
}

impl Neg for Bits8 {
    type Output = Bits8;

    fn neg(self) -> Self::Output {
        self
    }
}

impl Add for Bits8 {
    type Output = Bits8;

    fn add(self, other: Self) -> Self {
        self ^ other
    }
}

impl Sub for Bits8 {
    type Output = Bits8;

    fn sub(self, other: Self) -> Self {
        self ^ other
    }
}

impl Mul for Bits8 {
    type Output = Bits8;

    fn mul(self, other: Self) -> Self::Output {
        self & other
    }
}

impl Div for Bits8 {
    type Output = Bits8;

    fn div(self, other: Self) -> Self {
        assert_ne!(other, other.zero(), "Division of bits by Zero");
        self
    }
}
