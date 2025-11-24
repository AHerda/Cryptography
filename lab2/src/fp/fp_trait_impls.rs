use std::ops::{Add, Div, Mul, Rem, Sub};

use super::{Fp, ModType, P};

impl<T> Fp<T>
where
    T: Add + Rem<ModType, Output = T> + Div + Mul + Sub + Eq + Copy,
{
    pub fn new(number: T) -> Self {
        Self(number % P)
    }
}

impl<T> Add for Fp<T>
where
    T: Add<Output = T> + Rem<ModType, Output = T> + Div + Mul + Sub + Eq + Copy,
{
    type Output = Fp<T>;

    fn add(self, other: Self) -> Self {
        Self::new(self.0 + other.0)
    }
}

impl<T> Sub for Fp<T>
where
    T: Add
        + Add<ModType, Output = T>
        + Rem<ModType, Output = T>
        + Div
        + Mul
        + Sub<Output = T>
        + Eq
        + Copy,
{
    type Output = Fp<T>;

    fn sub(self, other: Self) -> Self {
        Self::new(self.0 + P - other.0)
    }
}

impl<T> Mul for Fp<T>
where
    T: Add + Rem<ModType, Output = T> + Div + Mul<Output = T> + Sub + Eq + Copy,
{
    type Output = Fp<T>;

    fn mul(self, other: Self) -> Self {
        Self::new(self.0 * other.0)
    }
}

impl<T> Div for Fp<T>
where
    T: Add + Rem<ModType, Output = T> + Div<Output = T> + Mul + Sub + Eq + Copy,
{
    type Output = Fp<T>;

    fn div(self, other: Self) -> Self {
        Self::new(self.0 / other.0)
    }
}
