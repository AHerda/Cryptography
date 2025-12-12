use super::traits::Field;

mod ec_point_impls;

/// Elliptic curve over a finite field.
/// Described by the equation y^2 = x^3 + ax + b.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Ec<T: Field> {
    pub a: T,
    pub b: T,
}

/// Point on an elliptic curve.
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum EcPoint<T: Field> {
    Point { x: T, y: T, ec: Ec<T> },
    Infinity,
}

#[derive(Debug)]
pub enum EcErrors {
    PointNotOnCurve,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fp::Fp;
    const P: usize = 19;

    #[test]
    fn additions_on_infinity() {
        let p1: Fp<P> = Fp::new(5);
        let p2: Fp<P> = Fp::new(10);
        let ec = Ec::new(p1, p2);
        let point = EcPoint::new(p1, p2, ec).expect("Point on curve");
        let inf: EcPoint<Fp<P>> = EcPoint::infinity();

        assert_eq!(inf, inf);
        assert_eq!(inf.clone() + inf.clone(), inf);
        assert_eq!(point.clone() + inf.clone(), point);
        assert_eq!(inf.clone() + point.clone(), point);
    }

    #[test]
    fn additions_on_point() {
        let p1: Fp<P> = Fp::new(5);
        let p2: Fp<P> = Fp::new(10);
        let p3: Fp<P> = Fp::new(9);
        let ec = Ec::new(p1, p2);
        let point1 = EcPoint::new(p1, p2, ec.clone()).expect("Point on curve");
        let point2 = EcPoint::new(p2, p1, ec.clone()).expect("Point on curve");
        let point_ecpected = EcPoint::new(p1, p3, ec).expect("Point on curve");
        let inf: EcPoint<Fp<P>> = EcPoint::infinity();

        // assert!(point1.is_on_curve());
        // assert!(point2.is_on_curve());
        // assert!(point_ecpected.is_on_curve());
        assert_eq!(point1, point1);
        assert_eq!(point1.clone() + (-point1.clone()), inf);
        assert_eq!(point1.clone() + point1.clone(), point1.double());
        assert_eq!(point1.clone() + point2.clone(), point_ecpected);
    }
}
