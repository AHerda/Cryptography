use std::ops::{Add, Div, Mul, Rem, Sub};

use super::{Fpk, K, ModType, P};
use crate::fp::{Fp, FpMustHave};

impl<T> Fpk<T>
where
    T: FpMustHave,
    ModType: Sub<T, Output = T>,
{
    pub fn new(coef: Vec<T>) -> Self {
        Self(coef.into_iter().map(Fp::new).collect())
    }

    fn degree(&self) -> Option<usize> {
        if self.0.is_empty() {
            None
        } else {
            Some(self.0.len() - 1)
        }
    }

    fn normalize(&mut self) {
        let zero_val = Fp::new(0.into());
        while let Some(last) = self.0.last() {
            if *last == zero_val {
                self.0.pop();
            } else {
                break;
            }
        }
    }

    fn div_rem(self, rhs: Self) -> (Self, Self) {
        let mut remainder = self;
        remainder.normalize();

        let mut divisor = rhs;
        divisor.normalize();

        if divisor.0.is_empty() {
            panic!("Division by zero polynomial");
        }

        let rhs_deg = divisor.degree().unwrap();
        let mut quotient_vec = vec![Fp::new(T::from(0)); std::cmp::max(1, remainder.0.len())];

        // While degree(remainder) >= degree(divisor)
        while let Some(rem_deg) = remainder.degree() {
            if rem_deg < rhs_deg {
                break;
            }

            let deg_diff = rem_deg - rhs_deg;

            // scale = lead(remainder) / lead(divisor)
            let lead_rem = *remainder.0.last().unwrap();
            let lead_div = *divisor.0.last().unwrap();
            let scale = lead_rem / lead_div;

            // Update quotient
            // Note: In a proper vector implementation, we need to handle sizing
            if deg_diff >= quotient_vec.len() {
                quotient_vec.resize(deg_diff + 1, Fp::new(T::from(0)));
            }
            quotient_vec[deg_diff] = scale;

            // Subtract (divisor * scale * x^deg_diff) from remainder
            for (i, coeff) in divisor.0.iter().enumerate() {
                let target_idx = i + deg_diff;
                if target_idx < remainder.0.len() {
                    remainder.0[target_idx] = remainder.0[target_idx] - (*coeff * scale);
                }
            }

            remainder.normalize();
        }

        // Clean up quotient
        let mut quotient = Fpk(quotient_vec);
        quotient.normalize();

        (quotient, remainder)
    }
}

impl<T> Add for Fpk<T>
where
    T: FpMustHave,
    ModType: Sub<T, Output = T>,
{
    type Output = Fpk<T>;

    fn add(self, other: Self) -> Self {
        let mut result = Fpk((0..std::cmp::max(self.0.len(), other.0.len()))
            .map(|i| {
                if i < self.0.len() && i < other.0.len() {
                    self.0[i] + other.0[i]
                } else if i < self.0.len() {
                    self.0[i]
                } else {
                    other.0[i]
                }
            })
            .collect::<Vec<Fp<T>>>());
        result.normalize();
        result
    }
}

impl<T> Sub for Fpk<T>
where
    T: FpMustHave,
    ModType: Sub<T, Output = T>,
{
    type Output = Fpk<T>;

    fn sub(self, other: Self) -> Self {
        let mut result = Fpk((0..std::cmp::max(self.0.len(), other.0.len()))
            .map(|i| {
                if i < self.0.len() && i < other.0.len() {
                    self.0[i] - other.0[i]
                } else if i < self.0.len() {
                    self.0[i]
                } else {
                    other.0[i]
                }
            })
            .collect::<Vec<Fp<T>>>());
        result.normalize();
        result
    }
}

impl<T> Mul for Fpk<T>
where
    T: FpMustHave,
    ModType: Sub<T, Output = T> + Copy,
{
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        if self.0.is_empty() || rhs.0.is_empty() {
            return Fpk(vec![]);
        }

        let new_len = self.0.len() + rhs.0.len() - 1;
        let mut result = vec![Fp::new(T::from(0)); new_len];

        for (i, a_val) in self.0.iter().enumerate() {
            for (j, b_val) in rhs.0.iter().enumerate() {
                result[i + j] = result[i + j] + (*a_val * *b_val);
            }
        }

        let mut fpk = Fpk(result);
        fpk.normalize();
        fpk
    }
}

impl<T> Div for Fpk<T>
where
    T: FpMustHave,
    ModType: Sub<T, Output = T>,
{
    type Output = Fpk<T>;

    fn div(self, other: Self) -> Self {
        self.div_rem(other).0
    }
}

impl<T> Rem for Fpk<T>
where
    T: FpMustHave,
    ModType: Sub<T, Output = T>,
{
    type Output = Fpk<T>;

    fn rem(self, other: Self) -> Self {
        self.div_rem(other).1
    }
}
