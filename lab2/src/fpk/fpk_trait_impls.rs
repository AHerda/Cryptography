use std::fmt::Display;
use std::ops::{Add, Div, Mul, Neg, Rem, Sub};

use super::{Fpk, T};
use crate::fp::Fp;
use crate::polynomials::Polynomial;
use crate::traits::Pow;

impl<const P: T, const K: T> Fpk<P, K> {
    pub fn new(poly: Polynomial<Fp<P>>, modulo: Polynomial<Fp<P>>) -> Self {
        assert_eq!(
            modulo.degree().expect("modulo must have a positive degree"),
            K
        );
        Self {
            poly: poly % modulo.clone(),
            modulo,
        }
    }

    fn is_zero(&self) -> bool {
        self.poly.is_zero()
    }

    fn match_mods(lhs: &Self, rhs: &Self) -> bool {
        lhs.modulo
            .coefficients()
            .iter()
            .zip(rhs.modulo.coefficients().iter())
            .all(|(a, b)| a == b)
    }

    fn degree(&self) -> Option<usize> {
        self.poly.degree()
    }

    pub fn coefficients(&self) -> Vec<Fp<P>> {
        self.poly.coefficients()
    }
}

impl<const P: T, const K: T> Pow for Fpk<P, K> {
    fn zero(&self) -> Self {
        Self::new(self.poly.zero(), self.modulo.clone())
    }
    fn one(&self) -> Self {
        Self::new(self.poly.one(), self.modulo.clone())
    }
}

impl<const P: T, const K: T> Display for Fpk<P, K> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.poly)
    }
}

impl<const P: T, const K: T> Neg for Fpk<P, K> {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            poly: (self.modulo.clone() - self.poly) % self.modulo.clone(),
            modulo: self.modulo,
        }
    }
}

impl<const P: T, const K: T> Add for Fpk<P, K> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        assert!(Self::match_mods(&self, &other));

        Self {
            poly: self.poly + other.poly,
            modulo: self.modulo,
        }
    }
}

impl<const P: T, const K: T> Sub for Fpk<P, K> {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        assert!(Self::match_mods(&self, &other));

        self + other.neg()
    }
}

impl<const P: T, const K: T> Mul for Fpk<P, K> {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        assert!(Self::match_mods(&self, &other));

        Self {
            poly: (self.poly * other.poly) % other.modulo,
            modulo: self.modulo,
        }
    }
}

impl<const P: T, const K: T> Div for Fpk<P, K> {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        assert!(!other.is_zero(), "Division of Fpk by zero");
        assert!(Self::match_mods(&self, &other));

        Self {
            poly: self.poly / other.poly,
            modulo: self.modulo,
        }
    }
}

impl<const P: T, const K: T> Rem for Fpk<P, K> {
    type Output = Self;

    fn rem(self, other: Self) -> Self {
        assert!(!other.is_zero(), "Reminder of Fpk by zero");
        assert!(Self::match_mods(&self, &other));

        Self {
            poly: self.poly % other.poly,
            modulo: self.modulo,
        }
    }
}
