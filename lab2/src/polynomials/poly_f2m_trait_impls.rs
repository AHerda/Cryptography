use std::fmt::Display;
use std::ops::{Div, Mul, Rem, Shl};

use super::Polynomial;
use crate::f2m::bit::Bits8;
use crate::traits::Pow;

impl Polynomial<Bits8> {
    pub fn degree(&self) -> Option<usize> {
        match self.coef.len() {
            0 => None,
            n => Some(self.coef[n - 1].0.ilog2() as usize + 8 * (n - 1)),
        }
    }

    pub(crate) fn div_rem(&self, rhs: &Self) -> (Self, Self) {
        let mut remainder = self.clone();
        let divisor = rhs.clone();

        if divisor.is_zero() {
            panic!("Division by zero");
        }

        let div_deg = divisor.degree().expect("Divisor degree should be positive");
        let mut quotient = self.zero();

        while let Some(rem_deg) = remainder.degree()
            && rem_deg >= div_deg
        {
            let deg_diff = rem_deg - div_deg;
            quotient.set_bit(deg_diff, true);

            remainder = remainder - (divisor.clone() << deg_diff);

            remainder.normalize();
        }

        quotient.normalize();

        (quotient, remainder)
    }

    pub fn get_bit(&self, index: usize) -> bool {
        let byte_index = index / 8;
        let bit_index = index % 8;

        if byte_index >= self.coef.len() {
            false
        } else {
            (self.coef[byte_index].0 & (1 << bit_index)) != 0
        }
    }

    fn set_bit(&mut self, index: usize, value: bool) {
        let byte_index = index / 8;
        let bit_index = index % 8;

        while self.coef.len() <= byte_index {
            self.coef.push(Bits8(0));
        }

        if value {
            self.coef[byte_index].0 |= 1 << bit_index;
        } else {
            self.coef[byte_index].0 &= !(1 << bit_index);
        }
    }
}

impl Display for Polynomial<Bits8> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_zero() {
            return write!(f, "0");
        }

        let degree = self.degree().unwrap() + 1;

        for i in (0..degree).rev() {
            let coef = self.coef[i / 8].0 & (1 << (i % 8));
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

impl Shl<usize> for Polynomial<Bits8> {
    type Output = Self;

    fn shl(mut self, shift: usize) -> Self::Output {
        if shift == 0 {
            return self;
        }
        if self.degree().is_none() {
            return self;
        }

        let n = self.coef.len();
        let shift_len = (shift / 8) + 1;
        self.coef.append(&mut vec![Bits8(0); shift_len]);

        let byte_shift = shift / 8;
        let bit_shift = shift % 8;
        for i in (0..n).rev() {
            let temp = self.coef[i].0;
            self.coef[i] = Bits8(0);
            self.coef[i + byte_shift] = Bits8(temp << bit_shift);
            self.coef[i + byte_shift + 1] = Bits8(if bit_shift > 0 {
                temp >> (8 - bit_shift)
            } else {
                0
            });
        }

        self.normalize();
        self
    }
}

impl Mul for Polynomial<Bits8> {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        if other.is_zero() {
            return self.zero();
        }
        let degree = other.degree().unwrap();

        let mut result = self.zero();
        let temp = self.clone();

        for i in 0..=degree {
            if other.get_bit(i) {
                result = result + (temp.clone() << i);
                result.normalize();
            }
        }

        result.normalize();
        result
    }
}

impl Div for Polynomial<Bits8> {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        assert!(!other.is_zero(), "Division of Polynomial by zero");
        self.div_rem(&other).0
    }
}

impl Rem for Polynomial<Bits8> {
    type Output = Self;

    fn rem(self, other: Self) -> Self {
        assert!(!other.is_zero(), "Reminder of Polynomial by zero");
        self.div_rem(&other).1
    }
}

impl Pow for Polynomial<Bits8> {
    fn zero(&self) -> Self {
        Self::new(vec![])
    }
    fn one(&self) -> Self {
        Self::new(vec![Bits8(1)])
    }
}
