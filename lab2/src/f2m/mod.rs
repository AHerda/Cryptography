use super::{fp::T, polynomials::Polynomial, traits::Pow};

mod bit;
mod f2m_trait_impls;

use bit::Bit;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct F2m<const M: T> {
    poly: Polynomial<Bit>,
    modulo: Polynomial<Bit>,
}

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

        let coeff_p1 = Polynomial::new(vec![One, Zero, One, One, Zero, One]);
        let coeff_p2 = Polynomial::new(vec![Zero, One, Zero, Zero, One]);
        let coeff_expected = Polynomial::new(vec![One, One, One, One, One, One]);

        let p1: F2m<M> = F2m::new(coeff_p1, pk.clone());
        let p2: F2m<M> = F2m::new(coeff_p2, pk.clone());
        let expected: F2m<M> = F2m::new(coeff_expected, pk);

        assert_eq!(p1.clone() + p1.clone(), p2.zero());
        assert_eq!(p1 + p2, expected);
    }

    // #[test]
    // fn test_mul() {
    //     const K: usize = 5;
    //     let pk: Polynomial<Fp<P>> = Polynomial::new(vec![
    //         Fp::new(1),
    //         Fp::new(2),
    //         Fp::new(3),
    //         Fp::new(1),
    //         Fp::new(2),
    //         Fp::new(3),
    //     ]);
    //     let coeff_p1 = Polynomial::new(vec![
    //         Fp::new(3),
    //         Fp::new(4),
    //         Fp::new(10),
    //         Fp::new(12),
    //         Fp::new(0),
    //         Fp::new(15),
    //     ]);
    //     let coeff_p2 = Polynomial::new(vec![
    //         Fp::new(6),
    //         Fp::new(16),
    //         Fp::new(2),
    //         Fp::new(1),
    //         Fp::new(0),
    //         Fp::new(11),
    //     ]);
    //     let coeff_expected = Polynomial::new(vec![
    //         Fp::new(10),
    //         Fp::new(4),
    //         Fp::new(6),
    //         Fp::new(15),
    //         Fp::new(2),
    //     ]);
    //     let p1: Fpk<P, K> = Fpk::new(coeff_p1, pk.clone());
    //     let p2: Fpk<P, K> = Fpk::new(coeff_p2, pk.clone());
    //     let expected: Fpk<P, K> = Fpk::new(coeff_expected, pk);

    //     assert_eq!(p1.clone() * p2, expected);
    // }
}
