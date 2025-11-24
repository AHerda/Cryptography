use std::ops::{Add, Div, Mul, Rem, Sub};

use super::fp::{Fp, FpMustHave};

mod fpk_trait_impls;

type ModType = u32;
const P: ModType = 19;
const K: ModType = 3;

#[derive(Debug, PartialEq, Eq, PartialOrd, Clone)]
pub struct Fpk<T>(Vec<Fp<T>>)
where
    T: FpMustHave,
    ModType: Sub<T, Output = T>;
