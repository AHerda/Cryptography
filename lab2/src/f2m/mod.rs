use super::{
    fp::T,
    polynomials::Polynomial,
    traits::{Field, Pow},
};

mod bit;
mod f2m_trait_impls;

use bit::Bit;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct F2m<const M: T> {
    poly: Polynomial<Bit>,
    modulo: Polynomial<Bit>,
}

impl<const M: T> Field for F2m<M> {}

#[cfg(test)]
mod tests {
    use super::{Bit::*, *};
    const M: T = 6;

    #[test]
    fn test_display() {
        let pk = Polynomial::new(vec![One, One, Zero, One, One, One, One]);
        let coeff = Polynomial::new(vec![One, Zero, One, One, Zero, One]);
        let f2m: F2m<M> = F2m::new(coeff, pk);

        assert_eq!(format!("{}", f2m), "x^5 + x^3 + x^2 + 1");
    }

    #[test]
    fn test_creation() {
        const M: usize = 3;
        let pk: Polynomial<Bit> = Polynomial::new(vec![One, Zero, One, One]);
        let polynomial = Polynomial::new(vec![One, Zero, One, One, Zero, One]);
        let p1: F2m<M> = F2m::new(polynomial, pk.clone());

        // Value calculated with wofram mathematica
        let polynomial = Polynomial::new(vec![One, One]);
        let expected: F2m<M> = F2m::new(polynomial, pk);

        assert_eq!(p1, expected);
    }

    #[test]
    fn test_creation2() {
        const M: usize = 3;
        let pk: Polynomial<Bit> = Polynomial::new(vec![One, Zero, One, One]);
        let polynomial = Polynomial::new(vec![One, Zero, Zero, Zero]);
        let p1: F2m<M> = F2m::new(polynomial, pk.clone());

        // Value calculated with wofram mathematica
        let polynomial = Polynomial::new(vec![One]);
        let expected: F2m<M> = F2m::new(polynomial, pk);

        assert_eq!(p1, expected);
    }

    #[test]
    fn test_add() {
        const M: usize = 7;
        let pk: Polynomial<Bit> = Polynomial::new(vec![One, Zero, One, One, Zero, One, Zero, One]);

        let coeff_p1 = Polynomial::new(vec![One, Zero, One, Zero, Zero, One]);
        let coeff_p2 = Polynomial::new(vec![Zero, One, One, Zero, One]);
        let coeff_expected = Polynomial::new(vec![One, One, Zero, Zero, One, One]);

        let p1: F2m<M> = F2m::new(coeff_p1, pk.clone());
        let p2: F2m<M> = F2m::new(coeff_p2, pk.clone());
        let expected: F2m<M> = F2m::new(coeff_expected, pk);

        assert_eq!(p1.clone() + p1.clone(), p2.zero());
        assert_eq!(p1 + p2, expected);
    }

    #[test]
    fn test_sub() {
        const M: usize = 7;
        let pk: Polynomial<Bit> = Polynomial::new(vec![One, Zero, One, One, Zero, One, Zero, One]);

        let coeff_p1 = Polynomial::new(vec![One, Zero, One, Zero, Zero, One]);
        let coeff_p2 = Polynomial::new(vec![Zero, One, One, Zero, One]);
        let coeff_expected = Polynomial::new(vec![One, One, Zero, Zero, One, One]);

        let p1: F2m<M> = F2m::new(coeff_p1, pk.clone());
        let p2: F2m<M> = F2m::new(coeff_p2, pk.clone());
        let expected: F2m<M> = F2m::new(coeff_expected, pk);

        assert_eq!(p1.clone() - p1.clone(), p2.zero());
        assert_eq!(p1 - p2, expected);
    }

    #[test]
    fn test_mul() {
        const M: T = 5;
        let pk: Polynomial<Bit> = Polynomial::new(vec![One, Zero, One, One, Zero, One]);
        let coeff_p1 = Polynomial::new(vec![One, Zero, One]);
        let coeff_p2 = Polynomial::new(vec![One, One, Zero, One]);
        // Values calculated using wolfram mathematica
        let coeff_expected = Polynomial::new(vec![Zero, One, Zero, One]);

        let p1: F2m<M> = F2m::new(coeff_p1, pk.clone());
        let p2: F2m<M> = F2m::new(coeff_p2, pk.clone());
        let expected: F2m<M> = F2m::new(coeff_expected, pk);

        assert_eq!(p1 * p2, expected);
    }

    #[test]
    fn test_division() {
        const M: T = 5;
        let pk: Polynomial<Bit> = Polynomial::new(vec![One, Zero, One, One, Zero, One]);
        let coeff_p1 = Polynomial::new(vec![One, One, Zero, One]);
        let coeff_p2 = Polynomial::new(vec![One, Zero, One]);
        // Values calculated using wolfram mathematica
        let coeff_expected = Polynomial::new(vec![Zero, One]);

        let p1: F2m<M> = F2m::new(coeff_p1, pk.clone());
        let p2: F2m<M> = F2m::new(coeff_p2, pk.clone());
        let expected: F2m<M> = F2m::new(coeff_expected, pk);

        assert_eq!(p1 / p2, expected);
    }

    #[test]
    fn test_remainder() {
        const M: T = 5;
        let pk: Polynomial<Bit> = Polynomial::new(vec![One, Zero, One, One, Zero, One]);
        let coeff_p1 = Polynomial::new(vec![One, One, Zero, One]);
        let coeff_p2 = Polynomial::new(vec![One, Zero, One]);
        // Values calculated using wolfram mathematica
        let coeff_expected = Polynomial::new(vec![One]);

        let p1: F2m<M> = F2m::new(coeff_p1, pk.clone());
        let p2: F2m<M> = F2m::new(coeff_p2, pk.clone());
        let expected: F2m<M> = F2m::new(coeff_expected, pk);

        assert_eq!(p1 % p2, expected);
    }
}
