use std::fmt::Display;
use std::ops::{Add, Div, Mul, Sub};

pub trait Field:
    Clone
    + PartialEq
    + Eq
    + Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
    // + Rem<Output = Self>
    + Display
    + Pow
{
}
pub trait Pow: std::ops::Mul<Output = Self> + Clone {
    /// Returns the additive identity element of the type.
    fn zero(&self) -> Self;
    /// Returns the multiplicative identity element of the type.
    fn one(&self) -> Self;
    /// Returns the power of the base to the exponent.
    /// Works by using the exponentiation by squaring algorithm.
    fn pow(self, mut exp: usize) -> Self {
        let mut base = self.clone();
        let mut result = self.one();

        while exp > 0 {
            if exp % 2 == 1 {
                result = result * base.clone();
            }
            base = base.clone() * base;
            exp /= 2;
        }

        result
    }
}

impl Pow for usize {
    fn zero(&self) -> Self {
        0
    }
    fn one(&self) -> Self {
        1
    }
}

impl Field for usize {}

impl Pow for isize {
    fn zero(&self) -> Self {
        0
    }
    fn one(&self) -> Self {
        1
    }
}

impl Field for isize {}
