extern crate lab2;

use lab2::{
    f2m::{F2m, bit::Bits8},
    polynomials::Polynomial,
};

const P: usize = 19;

fn main() {
    const M: usize = 12;
    let pk = Polynomial::new(vec![Bits8(0b11110111), Bits8(0b00010000)]);
    for i in 0..M {
        println!("i = {}", i);
        let mut number1: u8 = 1;
        let mut number2: u8 = if i < 8 { 0 } else { 1 };
        for j in 0..i {
            println!("\tj = {}", j);
            if j < 8 {
                number1 <<= 1;
                number1 += 1;
            } else if j < 16 {
                number2 <<= 1;
                number2 += 1;
            }
        }
        let coeff = Polynomial::new(vec![Bits8(number1), Bits8(number2)]);
        let f2m: F2m<M> = F2m::new(coeff, pk.clone());
        assert_eq!(f2m.degree(), Some(i));
    }
}
