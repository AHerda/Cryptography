use std::fmt::Display;
use std::ops::{Add, Div, Mul, Sub};

use super::{Fp, T};
use crate::traits::Pow;

impl<const P: T> Fp<P> {
    pub const fn new(number: T) -> Self {
        Self(number % P)
    }

    pub fn negative(&self) -> Self {
        Self::new(P - self.0)
    }

    pub fn inverse(&self) -> Self {
        let zero = self.0.zero();
        for i in (1..=(self.0 * (P - 1))).step_by(P) {
            if i % self.0 == zero {
                return Self(i / self.0);
            }
        }
        panic!("Cannot divide by zero")
    }
}

impl<const P: T> Pow for Fp<P> {
    fn zero(&self) -> Self {
        Fp::new(0)
    }

    fn one(&self) -> Self {
        Fp::new(1)
    }
}

impl<const P: T> Display for Fp<P> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<const P: T> Add for Fp<P> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self::new(self.0 + other.0)
    }
}

impl<const P: T> Sub for Fp<P> {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        self + other.negative()
    }
}

impl<const P: T> Mul for Fp<P> {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self::new(self.0 * other.0)
    }
}

impl<const P: T> Div for Fp<P> {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        self * other.inverse()
    }
}
