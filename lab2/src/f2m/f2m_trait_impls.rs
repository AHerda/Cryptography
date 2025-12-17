use std::fmt::Display;
use std::ops::{Add, Div, Mul, Neg, Rem, Sub};

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
        let pol_deg_option = Self::poly_degree(&poly);

        if let Some(pol_deg) = pol_deg_option && pol_deg < mod_deg {
            Self { poly, modulo }
        } else {
            Self {
                poly: poly % modulo.clone(),
                modulo,
            }
        }
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

    fn get_bit(&self, index: usize) -> bool {
        let coef = self.coefficients();
        let byte_index = index / 8;
        let bit_index = index % 8;

        if byte_index >= coef.len() {
            false
        } else {
            (coef[byte_index].0 & (1 << bit_index)) != 0
        }
    }

    fn get_bit_from_modulo(&self, index: usize) -> bool {
        let coef = self.modulo.coefficients();
        let byte_index = index / 8;
        let bit_index = index % 8;

        if byte_index >= coef.len() {
            false
        } else {
            (coef[byte_index].0 & (1 << bit_index)) != 0
        }
    }

    fn shift_left(self, shift: usize) -> Self {
        if shift == 0 {
            return self;
        }

        let mut coef = self.coefficients();
        let n = coef.len();
        for _ in 0..((shift - 1) / 8) {
            coef.push(Bits8(0));
        }

        for i in (0..n).rev() {
            let byte_shitf = shift / 8;
            let bit_shift = shift % 8;

            coef[i + byte_shitf] = Bits8( coef[i].0 << bit_shift );
            coef[i + byte_shitf + 1] = Bits8( coef[i].0 >> (8 - bit_shift) );
        }

        Self::new(Polynomial::new(coef), self.modulo.clone())
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

impl<const M: T> Neg for F2m<M> {
    type Output = Self;

    fn neg(self) -> Self {
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

    fn sub(self, other: Self) -> Self {
        assert!(Self::match_mods(&self, &other));

        self + other.neg()
    }
}

impl<const M: T> Mul for F2m<M> {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        assert!(Self::match_mods(&self, &other));

        let mut result = self.zero();
        let mut temp = self.clone();

        for i in 0..M {
            if other.get_bit(i) {
                result = result + temp;
            }

            // Multiply temp by X (shift left by 1)
            let overflow = temp.get_bit(M - 1);
            temp = temp.shift_left(1);

            if overflow {
                // Reduce by subtracting (XOR) irreducible polynomial
                for j in 0..=M {
                    if self.get_bit_from_modulo(j) {
                        let current = temp.get_bit(j);
                        temp.set_bit(j, current ^ true);
                    }
                }
            }

            temp.reduce();
        }

        result.reduce();
        result
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
