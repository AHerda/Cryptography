use rayon::prelude::*;
use std::time::Duration;

use lab2::{
    T,
    elliptic_curve::{Ec, EcPoint},
    f2m::{F2m, bit::Bits8},
};
use lab3::{
    diffie_hellman::{DiffieHellman, EcPointParams, F2mParams},
    schnorr::{ToJsonSchnorr, sign, verify},
};
use rand::Rng;

fn main() {
    const M1: T = 3072;
    const M2: T = 128;
    let random = 183;
    let mut rng = rand::rng();
    let m: String = (0..30).map(|_| rng.random::<char>()).collect();

    let mut modulo1: Vec<Bits8> = (0..M1 / 8).map(|_| Bits8(rng.random::<u8>())).collect();
    modulo1.push(Bits8(1));
    let poly1: Vec<Bits8> = (0..=256 / 8).map(|_| Bits8(rng.random::<u8>())).collect();
    let g = F2m::<M1>::new_from_slice(&poly1, &modulo1);
    let params: F2mParams<M1> = F2mParams {
        m: M1,
        g: g,
        q: random,
    };

    let avg = (0..1000).into_par_iter().reduce(
        || 0_u128,
        |acc, _i| acc + test_time::<F2m<M1>>(&params, random, &m).as_micros() as u128,
    );
    println!("f2m time: {:.02}", avg as f64 / 1000.0);

    let mut modulo2: Vec<Bits8> = (0..M2 / 8).map(|_| Bits8(rng.random::<u8>())).collect();
    modulo2.push(Bits8(1));

    let poly2_a: Vec<Bits8> = (0..=256 / 8).map(|_| Bits8(rng.random::<u8>())).collect();
    let a = F2m::<M2>::new_from_slice(&poly2_a, &modulo2);
    let poly2_b: Vec<Bits8> = (0..=256 / 8).map(|_| Bits8(rng.random::<u8>())).collect();
    let b = F2m::<M2>::new_from_slice(&poly2_b, &modulo2);
    let ec = Ec::<F2m<M2>>::new(a.clone(), b.clone());

    let poly2_gx: Vec<Bits8> = (0..=256 / 8).map(|_| Bits8(rng.random::<u8>())).collect();
    let x = F2m::<M2>::new_from_slice(&poly2_gx, &modulo2);
    let poly2_gy: Vec<Bits8> = (0..=256 / 8).map(|_| Bits8(rng.random::<u8>())).collect();
    let y = F2m::<M2>::new_from_slice(&poly2_gy, &modulo2);
    let g = EcPoint::new(x, y, ec).unwrap();

    let params: EcPointParams<F2m<M2>> = EcPointParams { a, b, g, q: random };

    let avg = (0..1000).into_par_iter().reduce(
        || 0_u128,
        |acc, _i| acc + test_time::<EcPoint<F2m<M2>>>(&params, random, &m).as_micros() as u128,
    );
    println!("ec time:  {:.02?}", avg as f64 / 1000.0);
}

fn test_time<Y: DiffieHellman + ToJsonSchnorr>(params: &Y::Params, random: T, m: &str) -> Duration {
    let start = std::time::Instant::now();
    let (s, e, _) = sign::<Y>(params, random, m);
    let _verified = verify::<Y>(&params, random, (s, e), m);
    let end = std::time::Instant::now();

    // if !verified {
    //     println!("Verification failed, m = {}", m);
    // }
    end - start
}
