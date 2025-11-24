use std::ops::{Add, Div, Mul, Rem, Sub};

mod fp_trait_impls;

type ModType = u32;
const P: ModType = 19;

pub trait FpMustHave:
    Add<Output = Self>
    + Rem<ModType, Output = Self>
    + Div<Output = Self>
    + Mul<Output = Self>
    + Sub<Output = Self>
    + Eq
    + Copy
    + Default
    + From<ModType>
{
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Copy, Clone)]
pub struct Fp<T: FpMustHave>(T);

impl FpMustHave for ModType {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let one = Fp::new(17);
        let two = Fp::new(18);

        assert_eq!(one + two, Fp::new(16));
    }

    #[test]
    fn test_sub() {
        let one = Fp::new(17);
        let two = Fp::new(18);

        assert_eq!(one - two, Fp::new(18));
    }

    #[test]
    fn test_mul() {
        let one = Fp::new(17);
        let two = Fp::new(18);

        assert_eq!(one * two, Fp::new(2));
    }

    #[test]
    fn test_div() {
        let one = Fp::new(18);
        let two = Fp::new(2);

        assert_eq!(one / two, Fp::new(9));
    }
}
