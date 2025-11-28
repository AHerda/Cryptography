use std::fmt::Display;
use std::ops::{Add, Div, Mul, Rem, Sub};

use super::{Fp, T};
use crate::pow_trait::Pow;

impl<const P: T> Fp<P> {
    pub const fn new(number: T) -> Self {
        Self(number % P)
    }

    pub fn negative(&self) -> Self {
        Self::new(P - self.0)
    }

    pub fn inverse(&self) -> Self {
        for i in 0..P {
            if self.0 * i == 1 {
                return Self::new(i.into());
            }
        }
        unreachable!()
    }
}

impl<const P: T> Pow for Fp<P> {
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
    type Output = Fp<P>;

    fn add(self, other: Self) -> Self {
        Self::new(self.0 + other.0)
    }
}

impl<const P: T> Sub for Fp<P> {
    type Output = Fp<P>;

    fn sub(self, other: Self) -> Self {
        self + other.negative()
    }
}

impl<const P: T> Mul for Fp<P> {
    type Output = Fp<P>;

    fn mul(self, other: Self) -> Self {
        Self::new(self.0 * other.0)
    }
}

impl<const P: T> Div for Fp<P>
// where
//     T: FpMustHave,
//     ModType: Sub<T, Output = T>,
{
    type Output = Fp<P>;

    fn div(self, other: Self) -> Self {
        self * other.inverse()
    }
}
