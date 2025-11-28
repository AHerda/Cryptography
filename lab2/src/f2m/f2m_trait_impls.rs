use std::fmt::Display;
use std::ops::{Add, Div, Mul, Rem, Sub};

use super::{F2m, T, Bit};
use crate::pow_trait::Pow;

impl<const M: T> F2m<M> {
    pub fn new(coef: Vec<Bit>, modulo: [Bit; M]) -> Self {
        Self { poly: coef, modulo }
    }

    fn match_mods(lhs: &Self, rhs: &Self) -> bool {
        lhs.modulo.iter().zip(rhs.modulo.iter()).all(|(a, b)| a == b)
    }

    fn degree(&self) -> Option<usize> {
        if self.poly.is_empty() {
            None
        } else {
            Some(self.poly.len() - 1)
        }
    }

    pub fn coefficients(&self) -> Vec<Bit> {
        self.poly.clone()
    }

    fn normalize(&mut self) {
        while let Some(&last) = self.poly.last() {
            if *!last {
                self.poly.pop();
            } else {
                break;
            }
        }
    }

    // pub const fn create_unredeucable_polynomial() -> Self {

    // }

    fn div_rem(&self, rhs: &Self) -> (Self, Self) {
        let mut remainder = self.clone();
        remainder.normalize();

        let mut divisor = rhs.clone();
        divisor.normalize();

        if divisor.poly.is_empty() {
            panic!("Division by zero polynomial");
        }

        let rhs_deg = divisor.degree().unwrap();
        let mut quotient_vec = vec![Bit::Zero; std::cmp::max(1, remainder.poly.len())];

        // While degree(remainder) >= degree(divisor)
        while let Some(rem_deg) = remainder.degree() {
            if rem_deg < rhs_deg {
                break;
            }

            let deg_diff = rem_deg - rhs_deg;

            // scale = lead(remainder) / lead(divisor)
            let lead_rem = *remainder.poly.last().unwrap();
            let lead_div = *divisor.poly.last().unwrap();
            let scale = lead_rem / lead_div;

            // Update quotient
            // Note: In a proper vector implementation, we need to handle sizing
            if deg_diff >= quotient_vec.len() {
                quotient_vec.resize(deg_diff + 1, Bit::Zero);
            }
            quotient_vec[deg_diff] = scale;

            // Subtract (divisor * scale * x^deg_diff) from remainder
            for (i, coeff) in divisor.poly.iter().enumerate() {
                let target_idx = i + deg_diff;
                if target_idx < remainder.poly.len() {
                    remainder.poly[target_idx] = remainder.poly[target_idx] - (*coeff * scale);
                }
            }

            remainder.normalize();
        }

        // Clean up quotient
        let mut quotient = F2m{ poly: quotient_vec, modulo: self.modulo.clone() };
        quotient.normalize();

        (quotient, remainder.clone())
    }
}

impl<const M: T> Pow for F2m<M> {
    fn one(&self) -> Self {
        F2m::new(vec![Bit::One], self.modulo.clone())
    }
}

impl<const M: T> Display for F2m<M> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.poly.is_empty() {
            return write!(f, "0");
        }

        for (i, coeff) in self.poly.iter().enumerate().rev() {
            if *coeff == Bit::Zero {
                continue;
            }

            if i == self.poly.len() - 1 {
                write!(f, "{}", coeff)?;
            } else {
                write!(f, " + ")?;
            }

            if i > 0 {
                write!(f, "x")?;
                if i > 1 {
                    write!(f, "^{}", i)?;
                }
            }
        }
        Ok(())
    }
}

impl<const M: T> Add for F2m<M> {
    type Output = F2m<M>;

    fn add(self, other: Self) -> Self {
        assert!(Self::match_mods(&self, &other));

        let mut result = F2m{
            poly: (0..std::cmp::max(self.poly.len(), other.poly.len()))
                .map(|i| {
                    if i < self.poly.len() && i < other.poly.len() {
                        self.poly[i] + other.poly[i]
                    } else if i < self.poly.len() {
                        self.poly[i]
                    } else {
                        other.poly[i]
                    }
                })
                .collect::<Vec<Bit>>(),
            modulo: self.modulo.clone(),
        };
        result.normalize();
        result
    }
}

impl<const M: T> Sub for F2m<M> {
    type Output = F2m<M>;

    fn sub(self, other: Self) -> Self {
        assert!(Self::match_mods(&self, &other));

        let mut result = F2m{
            poly: (0..std::cmp::max(self.poly.len(), other.poly.len()))
                .map(|i| {
                    if i < self.poly.len() && i < other.poly.len() {
                        self.poly[i] - other.poly[i]
                    } else if i < self.poly.len() {
                        self.poly[i]
                    } else {
                        other.poly[i]
                    }
                })
                .collect::<Vec<Bit>>(),
            modulo: self.modulo.clone()
        };
        result.normalize();
        result
    }
}

impl<const M: T> Mul for F2m<M> {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        assert!(Self::match_mods(&self, &other));

        if self.poly.is_empty() || other.poly.is_empty() {
            return F2m{ poly: vec![], modulo: self.modulo.clone() };
        }

        let new_len = self.poly.len() + other.poly.len() - 1;
        let mut result = vec![Bit::Zero; new_len];

        for (i, a_val) in self.poly.iter().enumerate() {
            for (j, b_val) in other.poly.iter().enumerate() {
                result[i + j] = result[i + j] + (*a_val * *b_val);
            }
        }

        Self::new(result, self.modulo)
    }
}

impl<const M: T> Div for F2m<M> {
    type Output = F2m<M>;

    fn div(self, other: Self) -> Self {
        assert!(Self::match_mods(&self, &other));

        self.div_rem(&other).0
    }
}

impl<const M: T> Rem for F2m<M> {
    type Output = F2m<M>;

    fn rem(self, other: Self) -> Self {
        assert!(Self::match_mods(&self, &other));

        self.div_rem(&other).1
    }
}
