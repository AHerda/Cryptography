use std::ops::{Add, Mul, Neg, Sub};

use super::{Ec, EcErrors, EcPoint, Field};
use crate::{
    f2m::F2m,
    traits::{EcCalculations, Normal, Pow, Sqrt},
};

impl<T: Field> Ec<T> {
    pub fn new(a: T, b: T) -> Self {
        Self { a, b }
    }

    pub fn get_infinity(&self) -> EcPoint<T> {
        EcPoint::Infinity
    }
}

impl<T: Field + Normal> EcCalculations<T> for Ec<T> {
    fn get_point_on_curve(&self, x: T) -> Result<EcPoint<T>, EcErrors> {
        let y = (x.clone().pow(3) + self.a.clone() * x.clone() + self.b.clone()); //.sqrt();
        // match y {
        //     Some(val) => Ok(EcPoint::Point {
        //         x,
        //         y: val,
        //         ec: self.clone(),
        //     }),
        //     Option::None => Err(EcErrors::NoYValueForSpecifiedX),
        // }
        Ok(EcPoint::Point {
            x,
            y, //: val,
            ec: self.clone(),
        })
    }

    #[inline]
    fn is_point_on_curve(&self, point: &EcPoint<T>) -> bool {
        match point {
            EcPoint::Infinity => true,
            EcPoint::Point { x, y, ec } => {
                y.clone().pow(2) == ec.a.clone() * x.clone() + x.clone().pow(3) + ec.b.clone()
            }
        }
    }

    fn add_points(&self, (x1, y1): (T, T), (x2, y2): (T, T)) -> EcPoint<T> {
        let scale = (y1.clone() - y2) / (x1.clone() - x2.clone());
        let x = scale.clone() * scale.clone() - x1.clone() - x2;
        let y = scale * (x1 - x.clone()) - y1;
        EcPoint::Point {
            x,
            y,
            ec: self.clone(),
        }
    }

    fn double_point(&self, (x, y): (T, T)) -> EcPoint<T> {
        if y == y.zero() {
            return EcPoint::Infinity;
        }

        let x_sq = x.clone().pow(2);
        let scale = (x_sq.clone() + x_sq.clone() + x_sq + self.a.clone()) / (y.clone() + y.clone());
        let x_new = scale.clone() * scale.clone() - x.clone() - x.clone();
        let y_new = scale * (x.clone() - x_new.clone()) - y.clone();
        EcPoint::Point {
            x: x_new,
            y: y_new,
            ec: self.clone(),
        }
    }
}

impl<const M: usize> EcCalculations<F2m<M>> for Ec<F2m<M>> {
    fn is_point_on_curve(&self, point: &EcPoint<F2m<M>>) -> bool {
        match point {
            EcPoint::Infinity => true,
            EcPoint::Point { x, y, ec } => {
                y.clone().pow(2) + y.clone() * x.clone()
                    == x.clone().pow(3) + ec.a.clone() * x.clone().pow(2) + ec.b.clone()
            }
        }
    }

    fn get_point_on_curve(&self, _x: F2m<M>) -> Result<EcPoint<F2m<M>>, EcErrors> {
        todo!()
    }

    fn add_points(
        &self,
        (x1, y1): (F2m<M>, F2m<M>),
        (x2, y2): (F2m<M>, F2m<M>),
    ) -> EcPoint<F2m<M>> {
        let scale = (y1.clone() + y2) / (x1.clone() - x2.clone());
        let x = scale.clone().pow(2) + scale.clone() + self.a.clone() + x1.clone() + x2;
        let y = scale * (x1 + x.clone()) - x.clone() + y1;
        EcPoint::Point {
            x,
            y,
            ec: self.clone(),
        }
    }

    fn double_point(&self, (x, y): (F2m<M>, F2m<M>)) -> EcPoint<F2m<M>> {
        if x.is_zero() {
            return EcPoint::Infinity;
        }

        let scale = x.clone() + y.clone() / x.clone();
        let x_new = scale.clone().pow(2) + scale.clone() - self.a.clone();
        let y_new = scale * (x.clone() + x_new.clone()) + x_new.clone() + y.clone();
        EcPoint::Point {
            x: x_new,
            y: y_new,
            ec: self.clone(),
        }
    }
}

impl<T> EcPoint<T>
where
    T: Field,
    Ec<T>: EcCalculations<T>,
{
    pub fn new(x: T, y: T, ec: Ec<T>) -> Result<Self, EcErrors> {
        let point = Self::Point { x, y, ec };
        match point.is_on_curve() {
            _ => Ok(point),
            // true => Ok(point),
            // false => Err(EcErrors::PointNotOnCurve),
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
            EcPoint::Point { x, y, ec } => ec.double_point((x.clone(), y.clone())),
        }
    }

    pub fn is_on_curve(&self) -> bool {
        match self {
            Self::Infinity => true,
            Self::Point { x: _, y: _, ec } => ec.is_point_on_curve(self),
        }
    }
}

impl<T> Neg for EcPoint<T>
where
    T: Field + Normal,
    Ec<T>: EcCalculations<T>,
{
    type Output = Self;

    fn neg(self) -> Self {
        match self {
            EcPoint::Point { x, y, ec } => EcPoint::Point { x, y: -y, ec },
            catch_all => catch_all,
        }
    }
}

impl<const M: usize> Neg for EcPoint<F2m<M>> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        match self {
            EcPoint::Point { x, y, ec } => EcPoint::Point {
                x: x.clone(),
                y: x + y,
                ec,
            },
            catch_all => catch_all,
        }
    }
}

impl<T> Add for EcPoint<T>
where
    T: Field,
    Ec<T>: EcCalculations<T>,
    EcPoint<T>: Neg<Output = Self>,
{
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
            ) => ec.add_points((x1, y1), (x2, y2)),
        }
    }
}

impl<T> Sub for EcPoint<T>
where
    T: Field,
    EcPoint<T>: Neg<Output = Self> + Add<Output = Self>,
{
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        self + other.neg()
    }
}

impl<T> Mul for EcPoint<T>
where
    T: Field,
    EcPoint<T>: Add<Output = Self>,
{
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        self + other
    }
}

impl<T> Pow for EcPoint<T>
where
    T: Field,
    EcPoint<T>: Add<Output = Self>,
{
    fn zero(&self) -> Self {
        Self::Infinity
    }
    fn one(&self) -> Self {
        Self::Infinity
    }
}

impl<T> Mul<usize> for EcPoint<T>
where
    T: Field,
    EcPoint<T>: Add<Output = Self>,
{
    type Output = Self;

    fn mul(self, other: usize) -> Self::Output {
        self.pow(other)
    }
}
