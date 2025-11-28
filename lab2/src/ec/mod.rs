use std::ops::{Add, Sub, Mul, Div, Rem};
use super::pow_trait::Pow;

struct Ec<T>
where T: Add + Sub + Mul + Div + Rem + Pow {
    number: T
}