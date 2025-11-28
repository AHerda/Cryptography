use super::fp::{Fp, T}; //, FpMustHave};

mod fpk_trait_impls;

// type ModType = u32;
const P: T = 19;
const K: T = 3;
const PK: [Fp<P>; K] = [Fp::new(1), Fp::new(2), Fp::new(3)];

#[derive(Debug, PartialEq, Eq, PartialOrd, Clone)]
pub struct Fpk<const P: T, const K: T>{
    poly: Vec<Fp<P>>,
    modulo: [Fp<P>; K],
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display() {
        let coeff = vec![3, 4, 10, 1, 0, 2];
        let fpk: Fpk<P, K> = Fpk::new(coeff, PK);

        assert_eq!(format!("{}", fpk), "2*x^5 + x^3 + 10*x^2 + 4*x + 3");
    }
}
