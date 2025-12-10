use super::traits::Field;

pub enum EcPoint<T: Field> {
    Point { x: T, y: T },
    Infinity,
}

pub struct Ec<T: Field> {
    a: T,
    b: T,
}
