use super::traits::{Field, Pow, Sqrt};

mod fp_trait_impls;

pub type T = usize;

#[derive(Debug, PartialEq, Eq, PartialOrd, Copy, Clone)]
pub struct Fp<const P: T>(T);

impl<const P: T> Field for Fp<P> {}

#[cfg(test)]
mod tests {
    use super::*;
    const P: T = 19;

    #[test]
    fn test_creation() {
        let one: Fp<P> = Fp::new(100);

        assert_eq!(one, Fp::new(5));
    }

    #[test]
    fn test_negation() {
        let one: Fp<P> = Fp::new(5);

        assert_eq!(-one, Fp::new(14));
    }

    #[test]
    fn test_inverse() {
        let one: Fp<P> = Fp::new(100);

        assert_eq!(one.inverse(), Fp::new(4));
    }

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

    #[test]
    fn test_pow() {
        let one: Fp<P> = Fp::new(10);
        let expected_2: Fp<P> = Fp::new(5);
        let expected_3: Fp<P> = Fp::new(12);
        let expected_4: Fp<P> = Fp::new(6);

        assert_eq!(one.clone().pow(2), expected_2);
        assert_eq!(one.clone().pow(3), expected_3);
        assert_eq!(one.clone().pow(4), expected_4);
    }

    #[test]
    fn test_sqrt() {
        for i in 0..P {
            let expected: Fp<P> = Fp::new(i);
            if expected.pow((P - 1) / 2) == expected.one() {
                assert_eq!(expected.sqrt().unwrap().pow(2), expected)
            } else {
                assert_eq!(expected.sqrt(), None);
            }
        }
    }
}
