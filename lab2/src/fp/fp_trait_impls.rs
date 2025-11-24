use std::ops::{Add, Div, Mul, Rem, Sub};

use super::{Fp, FpMustHave, ModType, P};

impl<T> Fp<T>
where
    T: FpMustHave,
    ModType: Sub<T, Output = T>,
{
    pub fn new(number: T) -> Self {
        Self(number % P)
    }

    pub fn negative(&self) -> Self {
        Self::new(P - self.0)
    }

    pub fn inverse(&self) -> Self {
        for i in 0..P {
            if self.0 * Fp(i.into()).0 == Fp(T::from(1)).0 {
                return Self::new(i.into());
            }
        }
        unreachable!()
    }
}

impl<T> Add for Fp<T>
where
    T: FpMustHave,
    ModType: Sub<T, Output = T>,
{
    type Output = Fp<T>;

    fn add(self, other: Self) -> Self {
        Self::new(self.0 + other.0)
    }
}

impl<T> Sub for Fp<T>
where
    T: FpMustHave,
    ModType: Sub<T, Output = T>,
{
    type Output = Fp<T>;

    fn sub(self, other: Self) -> Self {
        self + other.negative()
    }
}

impl<T> Mul for Fp<T>
where
    T: FpMustHave,
    ModType: Sub<T, Output = T>,
{
    type Output = Fp<T>;

    fn mul(self, other: Self) -> Self {
        Self::new(self.0 * other.0)
    }
}

impl<T> Div for Fp<T>
where
    T: FpMustHave,
    ModType: Sub<T, Output = T>,
{
    type Output = Fp<T>;

    fn div(self, other: Self) -> Self {
        self * other.inverse()
    }
}
