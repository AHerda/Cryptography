extern crate lab2;

use lab2::{
    SERIALIZATION_FORMAT, elliptic_curve::{Ec, EcErrors, EcPoint}, f2m::{F2m, bit::Bits8}, fp::{self, Fp}, fpk::{self, Fpk}, polynomials::Polynomial, traits::Pow
};


fn main() -> std::io::Result<()> {
    // const P: usize = 19;
    // let fp_elem1: Fp<P> = Fp::new(10);
    // let serialized = serde_json::to_string(&fp_elem1)?;
    // println!("Fp element 1: {}", serialized);


    _ = test_real();
    Ok(())
}

fn test() -> Result<(), EcErrors> {
    const P: usize = 5;
    const K: usize = 2;
    let pk: Polynomial<Fp<P>> = Polynomial::new(vec![Fp::new(2), Fp::new(0), Fp::new(1)]);
    let q_y_poly: Polynomial<Fp<P>> = Polynomial::new(vec![Fp::new(0), Fp::new(1)]);
    let a_b_poly: Polynomial<Fp<P>> = Polynomial::new(vec![Fp::new(1)]);
    let a_b: Fpk<P, K> = Fpk::new(a_b_poly, pk.clone());
    let q_y: Fpk<P, K> = Fpk::new(q_y_poly, pk);
    let ec: Ec<Fpk<P, K>> = Ec::new(a_b.clone(), a_b.clone());
    let p = EcPoint::new(q_y.zero(), a_b.clone(), ec.clone())?;
    let q = EcPoint::new(a_b.clone(), q_y, ec.clone())?;

    println!("{:?}", p.clone() + p.clone());
    println!("{:?}", p.clone() + q.clone());
    println!("{:?}", q.clone() + q.clone());
    println!("{:?}", -q);
    Ok(())
}

fn test_real() -> Result<(), EcErrors> {
    const P: usize = 17;
    let a_b: Fp<P> = Fp::new(2);
    let ec: Ec<Fp<P>> = Ec::new(a_b.clone(), a_b);
    let p = EcPoint::new(Fp::new(0), Fp::new(6), ec.clone())?;
    let q = EcPoint::new(Fp::new(5), Fp::new(1), ec.clone())?;

    println!("{:?}", p.clone() * 3);
    println!("{:?}", p.clone() + p.clone());
    println!("{:?}", p.clone() + q.clone());
    println!("{:?}", q.clone() + q.clone());
    println!("{:?}", -q);
    Ok(())
}
