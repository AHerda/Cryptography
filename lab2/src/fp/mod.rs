use std::fmt::Display;
use std::ops::{Add, Div, Mul, Rem, Sub};

mod fp_trait_impls;

pub type T = usize;
const P: T = 19;

#[derive(Debug, PartialEq, Eq, PartialOrd, Copy, Clone)]
pub struct Fp<const P: T>(T);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display() {
        let one: Fp<P> = Fp::new(17);

        assert_eq!(format!("{}", one), "17");
    }

    #[test]
    fn test_add() {
        let one: Fp<P> = Fp::new(17);
        let two: Fp<P> = Fp::new(18);

        assert_eq!(one + two, Fp::new(16));
    }

    #[test]
    fn test_sub() {
        let one: Fp<P> = Fp::new(17);
        let two: Fp<P> = Fp::new(18);

        assert_eq!(one - two, Fp::new(18));
    }

    #[test]
    fn test_mul() {
        let one: Fp<P> = Fp::new(17);
        let two: Fp<P> = Fp::new(18);

        assert_eq!(one * two, Fp::new(2));
    }

    #[test]
    fn test_div() {
        let one: Fp<P> = Fp::new(18);
        let two: Fp<P> = Fp::new(2);

        assert_eq!(one / two, Fp::new(9));
    }
}
