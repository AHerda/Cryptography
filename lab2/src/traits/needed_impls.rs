use super::{Field, Pow, Sqrt};

// This is needed for tests in Fpk and Fp
impl Pow for usize {
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
