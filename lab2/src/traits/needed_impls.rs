use std::ops::{Div, Mul, Rem, Sub};

use crate::{T, traits::Inverse};

use super::{Field, Pow};

// This is needed for tests in Fpk and Fp
impl Pow for T {
    fn zero(&self) -> Self {
        0
    }
    fn one(&self) -> Self {
        1
    }
}

// These two are neede for tests in Polynomials
impl Pow for isize {
    fn zero(&self) -> Self {
        0
    }
    fn one(&self) -> Self {
        1
    }
}

impl Field for isize {}

impl Pow for i128 {
    fn zero(&self) -> Self {
        0
    }
    fn one(&self) -> Self {
        1
    }
}

impl Field for i128 {}

pub fn gcd<
    T: Pow + Rem<Output = T> + Div<Output = T> + Mul<Output = T> + Sub<Output = T> + PartialEq,
>(
    mut a: T,
    mut b: T,
) -> (T, T, T) {
    let (mut x, mut y, mut u, mut v) = (a.zero(), a.one(), a.one(), a.zero());
    while a != a.zero() {
        let (q, r) = (b.clone() / a.clone(), b.clone() % a.clone());
        let (m, n) = (x - u.clone() * q.clone(), y - v.clone() * q);
        (b, a, x, y, u, v) = (a, r, u, v, m, n);
    }
    (b, x, y)
}
