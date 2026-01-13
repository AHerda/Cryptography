use std::fmt::Display;
use std::ops::{Add, Div, Mul, Neg, Sub};

use crate::elliptic_curve::{EcErrors, EcPoint};

pub mod needed_impls;

pub trait Normal {}

pub trait Field:
    Clone
    + std::fmt::Debug
    + PartialEq
    + Eq
    + Neg<Output = Self>
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

pub trait Sqrt: Field {
    fn sqrt(self) -> Option<Self>;
}

pub trait EcCalculations<T: Field> {
    fn is_point_on_curve(&self, point: &EcPoint<T>) -> bool;
    fn get_point_on_curve(&self, x: T) -> Result<EcPoint<T>, EcErrors>;
    fn add_points(&self, p1: (T, T), p2: (T, T)) -> EcPoint<T>;
    fn double_point(&self, p: (T, T)) -> EcPoint<T>;
}
