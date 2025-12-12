use std::ops::{Add, Mul, Neg, Sub};

use super::{Ec, EcErrors, EcPoint, Field};
use crate::traits::{Pow, Sqrt};

impl<T: Field> Ec<T> {
    pub fn new(a: T, b: T) -> Self {
        Self { a, b }
    }

    // pub fn get_point_on_curve(&self, x: T) -> EcPoint<T> {
    //     let y = sqrt
    // }

    #[inline]
    pub fn is_point_on_curve(&self, point: &EcPoint<T>) -> bool {
        match point {
            EcPoint::Infinity => true,
            EcPoint::Point { x, y, ec } => {
                y.clone().pow(2) == ec.a.clone() * x.clone() + x.clone().pow(3) + ec.b.clone()
            }
        }
    }

    pub fn get_infinity(&self) -> EcPoint<T> {
        EcPoint::Infinity
    }
}

impl<T: Field + Sqrt> EcPoint<T> {
    pub fn new(x: T, y: T, ec: Ec<T>) -> Result<Self, EcErrors> {
        let point = Self::Point { x, y, ec };
        match point.is_on_curve() {
            true => Ok(point),
            false => Err(EcErrors::PointNotOnCurve),
        }
    }

    pub fn infinity() -> Self {
        Self::Infinity
    }

    #[inline]
    pub fn match_ec(lhs: &Self, rhs: &Self) -> bool {
        match (lhs, rhs) {
            (EcPoint::Infinity, _) => true,
            (_, EcPoint::Infinity) => true,
            (
                EcPoint::Point {
                    x: _,
                    y: _,
                    ec: ec1,
                },
                EcPoint::Point {
                    x: _,
                    y: _,
                    ec: ec2,
                },
            ) => ec1 == ec2,
        }
    }

    #[inline]
    pub fn double(&self) -> Self {
        match self {
            EcPoint::Infinity => EcPoint::Infinity,
            EcPoint::Point { x, y, ec } => {
                let x_sq = x.clone().pow(2);
                let scale =
                    (x_sq.clone() + x_sq.clone() + x_sq + ec.a.clone()) / (y.clone() + y.clone());
                let x_new = scale.clone() * scale.clone() - x.clone() - x.clone();
                let y_new = scale * (x.clone() - x.clone()) - y.clone();
                Self::Point {
                    x: x_new,
                    y: y_new,
                    ec: ec.clone(),
                }
            }
        }
    }

    pub fn is_on_curve(&self) -> bool {
        match self {
            Self::Infinity => true,
            Self::Point { x: _, y: _, ec } => ec.is_point_on_curve(self),
        }
    }
}

impl<T: Field + Sqrt> Neg for EcPoint<T> {
    type Output = Self;

    fn neg(self) -> Self {
        match self {
            EcPoint::Point { x, y, ec } => EcPoint::Point { x, y: -y, ec },
            catch_all => catch_all,
        }
    }
}

impl<T: Field + Sqrt> Add for EcPoint<T> {
    type Output = Self;

    fn add(self, other: EcPoint<T>) -> Self::Output {
        assert!(Self::match_ec(&self, &other), "Not compatible Ec");

        match (self, other) {
            (EcPoint::Infinity, p) => p,
            (p, EcPoint::Infinity) => p,
            (p1, p2) if p1 == p2.clone().neg() => EcPoint::Infinity,
            (p1, p2) if p1 == p2 => p1.double(),
            (
                EcPoint::Point { x: x1, y: y1, ec },
                EcPoint::Point {
                    x: x2,
                    y: y2,
                    ec: _,
                },
            ) => {
                let scale = (y1.clone() - y2) / (x1.clone() - x2.clone());
                let x = scale.clone() * scale.clone() - x1.clone() - x2;
                let y = scale * (x1 - x.clone()) - y1;
                Self::Point { x, y, ec }
            }
        }
    }
}

impl<T: Field + Sqrt> Sub for EcPoint<T> {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        self + other.neg()
    }
}

impl<T: Field + Sqrt> Mul for EcPoint<T> {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        self + other
    }
}

impl<T: Field + Sqrt> Pow for EcPoint<T> {
    fn zero(&self) -> Self {
        Self::Infinity
    }
    fn one(&self) -> Self {
        Self::Infinity
    }
}

impl<T: Field + Sqrt> Mul<usize> for EcPoint<T> {
    type Output = Self;

    fn mul(self, other: usize) -> Self::Output {
        self.pow(other)
    }
}
