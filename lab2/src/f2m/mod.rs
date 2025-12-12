use super::{
    fp::T,
    polynomials::Polynomial,
    traits::{Field, Pow},
};

pub mod bit;
mod f2m_trait_impls;

use bit::Bits8;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct F2m<const M: T> {
    poly: Polynomial<Bits8>,
    modulo: Polynomial<Bits8>,
}

impl<const M: T> Field for F2m<M> {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_degree() {
        const M: T = 12;
        let pk = Polynomial::new(vec![Bits8(0b11110111), Bits8(0b00010000)]);
        for i in 0..M {
            println!("{}", i);
            let mut number1: u8 = 1;
            let mut number2: u8 = if i < 8 { 0 } else { 1 };
            for j in 0..i {
                if j < 8 {
                    number1 <<= 1;
                    number1 += 1;
                } else if j < 16 {
                    number2 <<= 1;
                    number2 += 1;
                }
            }
            let coeff = Polynomial::new(vec![Bits8(number1), Bits8(number2)]);
            let f2m: F2m<M> = F2m::new(coeff, pk.clone());
            assert_eq!(f2m.degree(), Some(i));
        }
    }

    #[test]
    fn test_display() {
        const M: T = 6;
        let pk = Polynomial::new(vec![Bits8(0b1111011)]);
        let coeff = Polynomial::new(vec![Bits8(0b101101)]);
        let f2m: F2m<M> = F2m::new(coeff, pk);

        assert_eq!(format!("{}", f2m), "x^5 + x^3 + x^2 + 1");
    }

    #[test]
    fn test_creation() {
        const M: usize = 3;
        let pk: Polynomial<Bits8> = Polynomial::new(vec![Bits8(0b1011)]);
        let polynomial = Polynomial::new(vec![Bits8(0b101101)]);
        let p1: F2m<M> = F2m::new(polynomial, pk.clone());

        // Value calculated with wofram mathematica
        let polynomial = Polynomial::new(vec![Bits8(0b11)]);
        let expected: F2m<M> = F2m::new(polynomial, pk);

        assert_eq!(p1, expected);
    }

    #[test]
    fn test_creation2() {
        const M: usize = 3;
        let pk: Polynomial<Bits8> = Polynomial::new(vec![Bits8(0b1101)]);
        let polynomial = Polynomial::new(vec![Bits8(0b0001)]);
        let p1: F2m<M> = F2m::new(polynomial, pk.clone());

        // Value calculated with wofram mathematica
        let polynomial = Polynomial::new(vec![Bits8(0b1)]);
        let expected: F2m<M> = F2m::new(polynomial, pk);

        assert_eq!(p1, expected);
    }

    #[test]
    fn test_add() {
        const M: usize = 7;
        let pk: Polynomial<Bits8> = Polynomial::new(vec![Bits8(0b10101101)]);

        let coeff_p1 = Polynomial::new(vec![Bits8(0b100101)]);
        let coeff_p2 = Polynomial::new(vec![Bits8(0b10110)]);
        let coeff_expected = Polynomial::new(vec![Bits8(0b110011)]);

        let p1: F2m<M> = F2m::new(coeff_p1, pk.clone());
        let p2: F2m<M> = F2m::new(coeff_p2, pk.clone());
        let expected: F2m<M> = F2m::new(coeff_expected, pk);

        assert_eq!(p1.clone() + p1.clone(), p2.zero());
        assert_eq!(p1 + p2, expected);
    }

    #[test]
    fn test_sub() {
        const M: usize = 7;
        let pk: Polynomial<Bits8> = Polynomial::new(vec![Bits8(0b10101101)]);

        let coeff_p1 = Polynomial::new(vec![Bits8(0b100101)]);
        let coeff_p2 = Polynomial::new(vec![Bits8(0b10110)]);
        let coeff_expected = Polynomial::new(vec![Bits8(0b110011)]);

        let p1: F2m<M> = F2m::new(coeff_p1, pk.clone());
        let p2: F2m<M> = F2m::new(coeff_p2, pk.clone());
        let expected: F2m<M> = F2m::new(coeff_expected, pk);

        assert_eq!(p1.clone() - p1.clone(), p2.zero());
        assert_eq!(p1 - p2, expected);
    }

    #[test]
    fn test_mul() {
        const M: T = 5;
        let pk: Polynomial<Bits8> = Polynomial::new(vec![Bits8(0b101101)]);
        let coeff_p1 = Polynomial::new(vec![Bits8(0b101)]);
        let coeff_p2 = Polynomial::new(vec![Bits8(0b1011)]);
        // Values calculated using wolfram mathematica
        let coeff_expected = Polynomial::new(vec![Bits8(0b1010)]);

        let p1: F2m<M> = F2m::new(coeff_p1, pk.clone());
        let p2: F2m<M> = F2m::new(coeff_p2, pk.clone());
        let expected: F2m<M> = F2m::new(coeff_expected, pk);

        assert_eq!(p1 * p2, expected);
    }

    #[test]
    fn test_division() {
        const M: T = 5;
        let pk: Polynomial<Bits8> = Polynomial::new(vec![Bits8(0b101101)]);
        let coeff_p1 = Polynomial::new(vec![Bits8(0b1011)]);
        let coeff_p2 = Polynomial::new(vec![Bits8(0b101)]);
        // Values calculated using wolfram mathematica
        let coeff_expected = Polynomial::new(vec![Bits8(0b10)]);

        let p1: F2m<M> = F2m::new(coeff_p1, pk.clone());
        let p2: F2m<M> = F2m::new(coeff_p2, pk.clone());
        let expected: F2m<M> = F2m::new(coeff_expected, pk);

        assert_eq!(p1 / p2, expected);
    }

    #[test]
    fn test_remainder() {
        const M: T = 5;
        let pk: Polynomial<Bits8> = Polynomial::new(vec![Bits8(0b101101)]);
        let coeff_p1 = Polynomial::new(vec![Bits8(0b1011)]);
        let coeff_p2 = Polynomial::new(vec![Bits8(0b101)]);
        // Values calculated using wolfram mathematica
        let coeff_expected = Polynomial::new(vec![Bits8(0b1)]);

        let p1: F2m<M> = F2m::new(coeff_p1, pk.clone());
        let p2: F2m<M> = F2m::new(coeff_p2, pk.clone());
        let expected: F2m<M> = F2m::new(coeff_expected, pk);

        assert_eq!(p1 % p2, expected);
    }
}
