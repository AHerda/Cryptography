use std::fmt::Display;
use std::ops::{Add, Div, Mul, Neg, Rem, Sub, Shl};

use super::{Bits8, F2m, T};
use crate::polynomials::Polynomial;
use crate::traits::Pow;

impl<const M: T> F2m<M> {
    pub fn new(poly: Polynomial<Bits8>, modulo: Polynomial<Bits8>) -> Self {
        let mod_deg = Self::poly_degree(&modulo).expect("Modulo must have a positive degree");
        assert_eq!(
            mod_deg,
            M
        );
        
        let mut result = Self { poly, modulo };
        result.reduce();
        result
    }

    #[inline]
    fn match_mods(lhs: &Self, rhs: &Self) -> bool {
        lhs.modulo
            .coefficients()
            .iter()
            .zip(rhs.modulo.coefficients().iter())
            .all(|(a, b)| a == b)
    }

    #[inline]
    pub fn is_zero(&self) -> bool {
        self.poly.is_zero()
    }

    fn poly_degree(poly: &Polynomial<Bits8>) -> Option<usize> {
        let coef = poly.coefficients();
        match coef.len() {
            0 => None,
            n => Some(coef[n - 1].0.ilog2() as usize + 8 * (n - 1)),
        }
    }

    pub fn degree(&self) -> Option<usize> {
        Self::poly_degree(&self.poly)
    }

    #[inline]
    pub fn coefficients(&self) -> Vec<Bits8> {
        self.poly.coefficients()
    }

    // pub const fn create_unredeucable_polynomial() -> Self {

    // }

    fn _get_bit(&self, index: usize) -> bool {
        self.poly.get_bit(index)
    }

    fn _get_bit_from_modulo(&self, index: usize) -> bool {
        let coef = self.modulo.coefficients();
        let byte_index = index / 8;
        let bit_index = index % 8;

        if byte_index >= coef.len() {
            false
        } else {
            (coef[byte_index].0 & (1 << bit_index)) != 0
        }
    }

    fn reduce(&mut self) {
        self.poly = Self::div_rem_poly(&self.poly, &self.modulo).1;
    }

    fn div_rem_poly(lhs: &Polynomial<Bits8>, rhs: &Polynomial<Bits8>) -> (Polynomial<Bits8>, Polynomial<Bits8>) {
        lhs.div_rem(rhs)
    }
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

        let degree = self.degree().unwrap() + 1;
        let coefficients = self.poly.coefficients();

        for i in (0..degree).rev() {
            let coef = coefficients[i / 8].0 & (1 << (i % 8));
            if coef == 0 {
                continue;
            }

            if i != degree - 1 {
                write!(f, " + ")?;
            }

            if i > 0 {
                write!(f, "x")?;
                if i > 1 {
                    write!(f, "^{}", i)?;
                }
            } else if coef != 0 {
                write!(f, "{}", coef)?;
            }
        }
        Ok(())
    }
}

impl<const M: T> Shl<usize> for F2m<M> {
    type Output = Self;

    fn shl(mut self, shift: usize) -> Self::Output {
        self.poly = self.poly.clone() << shift;
        self
    }
}

impl<const M: T> Neg for F2m<M> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        self.clone()
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

    fn sub(self, other: Self) -> Self::Output {
        assert!(Self::match_mods(&self, &other));

        self + other.neg()
    }
}

impl<const M: T> Mul for F2m<M> {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        assert!(Self::match_mods(&self, &other));

        Self::new(self.poly * other.poly, other.modulo)
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
