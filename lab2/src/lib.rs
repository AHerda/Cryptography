use num_bigint::BigUint;

pub mod elliptic_curve;
pub mod f2m;
pub mod fp;
pub mod fpk;
pub mod polynomials;
pub mod traits;

pub type T = u128;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FieldFormat {
    Decimal,
    Hex,
    Base64,
}

thread_local! {
    pub static SERIALIZATION_FORMAT: std::cell::Cell<FieldFormat> = std::cell::Cell::new(FieldFormat::Decimal);
}
