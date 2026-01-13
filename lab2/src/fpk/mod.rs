use super::fp::Fp;
use super::traits::{Field, Normal};
use crate::T;
use crate::polynomials::Polynomial;

pub use fpk_serde::deser;

mod fpk_serde;
mod fpk_trait_impls;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Fpk<const P: T, const K: T> {
    poly: Polynomial<Fp<P>>,
    modulo: Polynomial<Fp<P>>,
}

impl<const P: T, const K: T> Field for Fpk<P, K> {}
impl<const P: T, const K: T> Normal for Fpk<P, K> {}

#[cfg(test)]
mod tests {
    use super::*;
    const P: T = 19;

    #[test]
    fn test_creation() {
        const K: T = 2;
        let pk: Polynomial<Fp<P>> = Polynomial::new(vec![Fp::new(1), Fp::new(2), Fp::new(3)]);
        let polynomial = Polynomial::new(vec![Fp::new(3), Fp::new(4), Fp::new(10), Fp::new(1)]);
        let p1: Fpk<P, K> = Fpk::new(polynomial, pk.clone());

        // Value calculated with wofram mathematica
        let polynomial = Polynomial::new(vec![Fp::new(2), Fp::new(8)]);
        let expected: Fpk<P, K> = Fpk::new(polynomial, pk);

        assert_eq!(p1, expected);
    }

    #[test]
    fn test_display() {
        const K: T = 8;
        let pk: Polynomial<Fp<P>> = Polynomial::new(vec![
            Fp::new(1),
            Fp::new(2),
            Fp::new(3),
            Fp::new(1),
            Fp::new(2),
            Fp::new(3),
            Fp::new(1),
            Fp::new(2),
            Fp::new(3),
        ]);
        let coeff = Polynomial::new(vec![
            Fp::new(3),
            Fp::new(4),
            Fp::new(10),
            Fp::new(1),
            Fp::new(0),
            Fp::new(2),
        ]);
        let p1: Fpk<P, K> = Fpk::new(coeff, pk);

        assert_eq!(format!("{}", p1), "2*x^5 + x^3 + 10*x^2 + 4*x + 3");
    }

    #[test]
    fn test_add() {
        const K: T = 8;
        let pk: Polynomial<Fp<P>> = Polynomial::new(vec![
            Fp::new(1),
            Fp::new(2),
            Fp::new(3),
            Fp::new(1),
            Fp::new(2),
            Fp::new(3),
            Fp::new(1),
            Fp::new(2),
            Fp::new(3),
        ]);
        let coeff_p1 = Polynomial::new(vec![
            Fp::new(3),
            Fp::new(4),
            Fp::new(10),
            Fp::new(12),
            Fp::new(0),
            Fp::new(15),
        ]);
        let coeff_expected = Polynomial::new(vec![
            Fp::new(6),
            Fp::new(8),
            Fp::new(1),
            Fp::new(5),
            Fp::new(0),
            Fp::new(11),
        ]);
        let p1: Fpk<P, K> = Fpk::new(coeff_p1, pk.clone());
        let expected: Fpk<P, K> = Fpk::new(coeff_expected, pk);

        assert_eq!(p1.clone() + p1, expected);
    }

    #[test]
    fn test_subtraction() {
        const K: T = 19;
        let p1: Polynomial<Fp<K>> = Polynomial::new(vec![Fp::new(1), Fp::new(4), Fp::new(7)]);
        let p2 = Polynomial::new(vec![Fp::new(4), Fp::new(5), Fp::new(6)]);
        let expected = Polynomial::new(vec![Fp::new(16), Fp::new(18), Fp::new(1)]);
        assert_eq!(p1 - p2, expected);
    }

    #[test]
    fn test_mul() {
        const K: T = 5;
        let pk: Polynomial<Fp<P>> = Polynomial::new(vec![
            Fp::new(1),
            Fp::new(2),
            Fp::new(3),
            Fp::new(1),
            Fp::new(2),
            Fp::new(3),
        ]);
        let coeff_p1 = Polynomial::new(vec![
            Fp::new(3),
            Fp::new(4),
            Fp::new(10),
            Fp::new(12),
            Fp::new(0),
            Fp::new(15),
        ]);
        let coeff_p2 = Polynomial::new(vec![
            Fp::new(6),
            Fp::new(16),
            Fp::new(2),
            Fp::new(1),
            Fp::new(0),
            Fp::new(11),
        ]);
        let coeff_expected = Polynomial::new(vec![
            Fp::new(10),
            Fp::new(4),
            Fp::new(6),
            Fp::new(15),
            Fp::new(2),
        ]);
        let p1: Fpk<P, K> = Fpk::new(coeff_p1, pk.clone());
        let p2: Fpk<P, K> = Fpk::new(coeff_p2, pk.clone());
        let expected: Fpk<P, K> = Fpk::new(coeff_expected, pk);

        assert_eq!(p1.clone() * p2, expected);
    }

    #[test]
    fn test_division() {
        const K: T = 6;
        let pk: Polynomial<Fp<P>> = Polynomial::new(vec![
            Fp::new(1),
            Fp::new(2),
            Fp::new(3),
            Fp::new(1),
            Fp::new(2),
            Fp::new(3),
            Fp::new(1),
        ]);
        let p1: Polynomial<Fp<P>> = Polynomial::new(vec![
            Fp::new(8),
            Fp::new(0),
            Fp::new(13),
            Fp::new(0),
            Fp::new(1),
            Fp::new(1),
        ]);
        let p2 = Polynomial::new(vec![Fp::new(12), Fp::new(1), Fp::new(0), Fp::new(3)]);
        // Expected value calculated with wolfram mathematica
        let expected = Polynomial::new(vec![Fp::new(2), Fp::new(13), Fp::new(13)]);

        let p1: Fpk<P, K> = Fpk::new(p1, pk.clone());
        let p2: Fpk<P, K> = Fpk::new(p2, pk.clone());
        let expected: Fpk<P, K> = Fpk::new(expected, pk);

        assert_eq!(p1 / p2, expected);
    }

    #[test]
    fn test_remainder() {
        const K: T = 6;
        let pk: Polynomial<Fp<P>> = Polynomial::new(vec![
            Fp::new(1),
            Fp::new(2),
            Fp::new(3),
            Fp::new(1),
            Fp::new(2),
            Fp::new(3),
            Fp::new(1),
        ]);
        let p1: Polynomial<Fp<19>> = Polynomial::new(vec![
            Fp::new(8),
            Fp::new(0),
            Fp::new(13),
            Fp::new(0),
            Fp::new(1),
            Fp::new(1),
        ]);
        let p2 = Polynomial::new(vec![Fp::new(12), Fp::new(1), Fp::new(0), Fp::new(3)]);
        // Expected value calculated with wolfram mathematica
        let expected = Polynomial::new(vec![Fp::new(3), Fp::new(13), Fp::new(15)]);

        let p1: Fpk<P, K> = Fpk::new(p1, pk.clone());
        let p2: Fpk<P, K> = Fpk::new(p2, pk.clone());
        let expected: Fpk<P, K> = Fpk::new(expected, pk);

        assert_eq!(p1 % p2, expected);
    }
}
