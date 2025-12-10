extern crate lab2;

use lab2::{fp::Fp, polynomials::Polynomial};

const P: usize = 19;

fn main() {
    let p1: Polynomial<Fp<P>> = Polynomial::new(vec![
        Fp::new(8),
        Fp::new(0),
        Fp::new(13),
        Fp::new(0),
        Fp::new(1),
        Fp::new(1),
    ]);
    println!("{}", p1);
    let p2 = Polynomial::new(vec![Fp::new(12), Fp::new(1), Fp::new(0), Fp::new(3)]);
    println!("{}", p2);
    let expected = Polynomial::new(vec![Fp::new(18), Fp::new(1), Fp::new(1)]);
    assert_eq!(p1 % p2, expected);
}
