extern crate lab2;

use lab2::{
    f2m::{F2m, bit::Bits8}, fp::{self, Fp}, polynomials::Polynomial,
    fpk::{Fpk, self},
};

const P: usize = 19;

fn main() -> std::io::Result<()> {
    let fp_elem1: Fpk<P, 5> = Fpk::new(Polynomial::new(vec![
        Fp::new(3),
        Fp::new(4),
        Fp::new(10),
        Fp::new(1),
        Fp::new(0),
        Fp::new(2),
    ]), Polynomial::new(vec![
        Fp::new(1),
        Fp::new(2),
        Fp::new(3),
        Fp::new(1),
        Fp::new(2),
        Fp::new(3),
    ]));
    let serialized = serde_json::to_string(&fp_elem1)?;
    println!("Fp element 1: {}", serialized);
    const P: usize = fpk::deser(include_str!("data.json"), "P");
    const K: usize = fpk::deser(include_str!("data.json"), "K");
    let fp_elem2: Fpk<P, K> = serde_json::from_str(&serialized)?;
    let serialized2 = serde_json::to_string(&fp_elem2)?;
    println!("Fp element 2: {}", serialized2);

    println!("{}", fp_elem1);
    println!("{}", fp_elem2);



    Ok(())
}
