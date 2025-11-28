use super::fp::{Fp, T}; //, FpMustHave};

mod bit;
mod f2m_trait_impls;

use bit::Bit::{self, *};

const M: T = 4;
const PK: [Bit; M] = [One, One, Zero, One];

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct F2m<const M: T>{
    poly: Vec<Bit>,
    modulo: [Bit; M],
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display() {
        let coeff = vec![One, Zero, One, One, Zero, One];
        let fpk: F2m<M> = F2m::new(coeff, PK);

        assert_eq!(format!("{}", fpk), "x^5 + x^3 + x^2 + 1");
    }
}
