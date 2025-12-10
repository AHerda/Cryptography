use std::fmt::Display;
use std::ops::{Add, BitAnd, BitOr, BitXor, Deref, Div, Mul, Not, Sub};

use crate::traits::{Field, Pow};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Bit {
    Zero,
    One,
}

impl Field for Bit {}

impl Pow for Bit {
    fn one(&self) -> Self {
        One
    }

    fn zero(&self) -> Self {
        Zero
    }

    fn pow(self, exponent: usize) -> Self {
        if self == Zero && exponent == 0 {
            panic!("Zero raised to the power of zero is undefined");
        }
        self
    }
}

impl Display for Bit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
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

    fn div(self, _other: Self) -> Self {
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
