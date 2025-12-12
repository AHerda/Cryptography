use std::fmt::Display;
use std::ops::{Add, Div, Mul, Neg, Rem, Sub};

use super::{Bit, F2m, T};
use crate::polynomials::Polynomial;
use crate::traits::Pow;

impl<const M: T> F2m<M> {
    pub fn new(poly: Polynomial<Bit>, modulo: Polynomial<Bit>) -> Self {
        assert_eq!(
            modulo.degree().expect("modulo must have a positive degree"),
            M
        );
        Self {
            poly: poly % modulo.clone(),
            modulo,
        }
    }

    fn match_mods(lhs: &Self, rhs: &Self) -> bool {
        lhs.modulo
            .coefficients()
            .iter()
            .zip(rhs.modulo.coefficients().iter())
            .all(|(a, b)| a == b)
    }

    pub fn is_zero(&self) -> bool {
        self.poly.is_zero()
    }

    pub fn degree(&self) -> Option<usize> {
        self.poly.degree()
    }

    pub fn coefficients(&self) -> Vec<Bit> {
        self.poly.coefficients()
    }

    // pub const fn create_unredeucable_polynomial() -> Self {

    // }
}

impl<const M: T> Pow for F2m<M> {
    fn zero(&self) -> Self {
        Self::new(self.poly.zero(), self.modulo.clone())
    }
    fn one(&self) -> Self {
        Self::new(self.poly.one(), self.modulo.clone())
    }
}

impl<const M: T> Display for F2m<M> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_zero() {
            return write!(f, "0");
        }

        let coefficients = self.poly.coefficients();

        for (i, coef) in coefficients.iter().enumerate().rev() {
            if *coef == Bit::Zero {
                continue;
            }

            if i != coefficients.len() - 1 {
                write!(f, " + ")?;
            }

            if i > 0 {
                write!(f, "x")?;
                if i > 1 {
                    write!(f, "^{}", i)?;
                }
            } else if *coef != Bit::Zero {
                write!(f, "{}", coef)?;
            }
        }
        Ok(())
    }
}

impl<const M: T> Neg for F2m<M> {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            poly: (self.modulo.clone() - self.poly) % self.modulo.clone(),
            modulo: self.modulo,
        }
    }
}

impl<const M: T> Add for F2m<M> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        assert!(Self::match_mods(&self, &other));

        Self {
            poly: self.poly + other.poly,
            modulo: self.modulo,
        }
    }
}

impl<const M: T> Sub for F2m<M> {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        assert!(Self::match_mods(&self, &other));

        self + other.neg()
    }
}

impl<const M: T> Mul for F2m<M> {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        assert!(Self::match_mods(&self, &other));

        Self {
            poly: (self.poly * other.poly) % other.modulo,
            modulo: self.modulo,
        }
    }
}

impl<const M: T> Div for F2m<M> {
    type Output = F2m<M>;

    fn div(self, other: Self) -> Self {
        assert!(!other.is_zero(), "Division of F2m by zero");
        assert!(Self::match_mods(&self, &other));

        Self {
            poly: self.poly / other.poly,
            modulo: self.modulo,
        }
    }
}

impl<const M: T> Rem for F2m<M> {
    type Output = F2m<M>;

    fn rem(self, other: Self) -> Self {
        assert!(!other.is_zero(), "Reminder of F2m by zero");
        assert!(Self::match_mods(&self, &other));

        Self {
            poly: self.poly % other.poly,
            modulo: self.modulo,
        }
    }
}
