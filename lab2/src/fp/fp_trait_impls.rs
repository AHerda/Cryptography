use std::fmt::Display;
use std::ops::{Add, Div, Mul, Neg, Rem, Sub};

use super::{Fp, T};
use crate::traits::needed_impls::gcd;
use crate::traits::{Inverse, Pow, Sqrt};

impl<const P: T> From<T> for Fp<P> {
    fn from(value: T) -> Self {
        Fp::<P>::new(value)
    }
}

impl<const P: T> Fp<P> {
    pub const fn new(number: T) -> Self {
        Self((number % P))
    }

    // pub fn inverse(&self) -> Self {
    //     let one = 1;
    //     for i in (1..=(self.0 * (P - 1))).step_by(P as usize) {
    //         if i % self.0 == one {
    //             return Self(i / self.0);
    //         }
    //     }
    //     panic!("Cannot divide by zero")
    // }

    pub fn get(&self) -> T {
        self.0
    }
}

impl<const P: T> Pow for Fp<P> {
    fn zero(&self) -> Self {
        Fp(0)
    }

    fn one(&self) -> Self {
        Fp(1)
    }
}

impl<const P: T> Sqrt for Fp<P> {
    fn sqrt(self) -> Option<Self> {
        if self.pow((P - 1) / 2) != self.one() {
            None
        } else {
            Some(self.pow((P + 1) / 4))
        }
    }
}

impl<const P: T> Display for Fp<P> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<const P: T> Neg for Fp<P> {
    type Output = Self;

    fn neg(self) -> Self {
        Self::new(P - self.0)
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
        self + other.neg()
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
        assert_ne!(other, self.zero(), "Division of Polynomial by zero");
        self * dbg!(other.inv())
    }
}

impl<const P: T> Rem for Fp<P> {
    type Output = Self;

    fn rem(self, other: Self) -> Self {
        Self::new(self.0 % other.0)
    }
}

impl<const P: T> Inverse for Fp<P> {
    fn inv(self) -> Self {
        let (g, x, _) = dbg!(gcd(self.0 as i128, P as i128));
        assert_eq!(g, 1, "Element is not invertible");
        Self::new((x + P as i128) as u128)
    }
}
