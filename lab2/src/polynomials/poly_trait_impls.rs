use std::fmt::Display;
use std::ops::{Add, Div, Mul, Neg, Rem, Sub};

use super::{Field, Polynomial};
use crate::traits::{Pow, Sqrt};

impl<T: Field> Polynomial<T> {
    pub fn new(coef: Vec<T>) -> Self {
        let mut result = Self { coef };
        result.normalize();
        result
    }

    pub fn degree(&self) -> Option<usize> {
        match self.coef.len() {
            0 => None,
            n => Some(n - 1),
        }
    }

    pub fn is_zero(&self) -> bool {
        self.coef.is_empty() || self.coef.iter().all(|coef| *coef == coef.zero())
    }

    pub fn coefficients(&self) -> Vec<T> {
        self.coef.clone()
    }

    fn normalize(&mut self) {
        while let Some(last) = self.coef.last() {
            if *last == last.zero() {
                self.coef.pop();
            } else {
                break;
            }
        }
    }

    // pub const fn create_unredeucable_Polynomial<P>() -> Self {

    // }

    fn div_rem(&self, rhs: &Self) -> (Self, Self) {
        let mut remainder = self.clone();
        let divisor = rhs.clone();

        if divisor.is_zero() {
            panic!("Division by zero");
        }

        let rhs_deg = divisor.degree().expect("Divisor degree should be positive");
        let lead_div = divisor.coef.last().cloned().unwrap();
        let mut quotient = self.zero();

        // While degree(remainder) >= degree(divisor)
        while let Some(rem_deg) = remainder.degree()
            && rem_deg > 0
            && rem_deg >= rhs_deg
        {
            // scale = lead(remainder) / lead(divisor)
            let lead_rem = remainder.coef.last().cloned().unwrap();
            let scale = lead_rem / lead_div.clone();
            let mut scale_vec = vec![scale.zero(); rem_deg - rhs_deg];
            scale_vec.push(scale);
            let tmp = Polynomial::new(scale_vec);

            // Update quotient
            quotient = quotient + tmp.clone();

            // Subtract (divisor * scale * x^deg_diff) from remainder
            remainder = remainder - (divisor.clone() * tmp);

            remainder.normalize();
        }

        // Clean up quotient
        quotient.normalize();

        (quotient, remainder.clone())
    }
}

impl<T: Field> Display for Polynomial<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.coef.is_empty() {
            return write!(f, "0");
        }

        for (i, coef) in self.coef.iter().enumerate().rev() {
            if *coef == coef.zero() {
                continue;
            }

            if i != self.coef.len() - 1 {
                write!(f, " + ")?;
            }

            if *coef != coef.one() {
                write!(f, "{}", coef)?;
            }

            if i > 0 {
                if *coef != coef.one() {
                    write!(f, "*")?;
                }
                write!(f, "x")?;
                if i > 1 {
                    write!(f, "^{}", i)?;
                }
            }
        }
        Ok(())
    }
}

impl<T: Field> Neg for Polynomial<T> {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            coef: self.coef.into_iter().map(|coef| -coef).collect::<Vec<T>>(),
        }
    }
}

impl<T: Field> Add for Polynomial<T> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        if self.is_zero() {
            return other;
        } else if other.is_zero() {
            return self;
        }
        let zero = self.coef[0].zero();
        let mut result = Self {
            coef: (0..std::cmp::max(self.coef.len(), other.coef.len()))
                .map(|i| {
                    self.coef.get(i).cloned().unwrap_or(zero.clone())
                        + other.coef.get(i).cloned().unwrap_or(zero.clone())
                })
                .collect::<Vec<T>>(),
        };
        result.normalize();
        result
    }
}

impl<T: Field> Sub for Polynomial<T> {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        let mut result = self + other.neg();
        result.normalize();
        result
    }
}

impl<T: Field> Mul for Polynomial<T> {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        if self.coef.is_empty() || other.coef.is_empty() {
            return Self { coef: vec![] };
        }

        let new_len = self.coef.len() + other.coef.len() - 1;
        let mut result = vec![self.coef[0].zero(); new_len];

        for (i, a_val) in self.coef.into_iter().enumerate() {
            for (j, b_val) in other.coef.clone().into_iter().enumerate() {
                result[i + j] = result[i + j].clone() + (a_val.clone() * b_val);
            }
        }

        Self::new(result)
    }
}

impl<T: Field> Div for Polynomial<T> {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        assert!(!other.is_zero(), "Division of Polynomial by zero");
        self.div_rem(&other).0
    }
}

impl<T: Field> Rem for Polynomial<T> {
    type Output = Self;

    fn rem(self, other: Self) -> Self {
        assert!(!other.is_zero(), "Reminder of Polynomial by zero");
        self.div_rem(&other).1
    }
}

impl<T: Field> Pow for Polynomial<T> {
    fn zero(&self) -> Self {
        Self::new(vec![])
    }
    fn one(&self) -> Self {
        Self::new(vec![self.coef[0].one()])
    }
}
