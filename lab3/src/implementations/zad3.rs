use std::{
    collections::HashSet,
    ops::{Add, Mul, Neg},
    process::Output,
};

use lab2::{
    T,
    elliptic_curve::{Ec, EcPoint},
    f2m::F2m,
    fp::Fp,
    fpk::Fpk,
    traits::{EcCalculations, Field, Inverse, Pow},
};

use crate::schnorr::ToJsonSchnorr;

pub trait ParamsForDiffieHellman {
    type G: Pow + ToJsonSchnorr + Inverse;
    fn get_g(&self) -> Self::G;
    fn get_q(&self) -> T;
}

pub trait DiffieHellman {
    type Params: ParamsForDiffieHellman;
    // type SecretKey;
    type PublicKey;

    // fn order_of_group_of_g(g: &Self::G) -> T {
    //     let mut temp = g.clone();
    //     let mut seen = vec![];
    //     while !seen.contains(&temp) && temp != temp.one() {
    //         seen.push(temp.clone());
    //         temp = temp * g.clone();
    //     }
    //     seen.len() as T + 1
    // }
    fn generate_secret_key(params: &Self::Params, random_value: T) -> T;
    fn compute_public_key(params: &Self::Params, secret_key: &T) -> Self::PublicKey;
    fn compute_shared_secret(
        params: &Self::Params,
        secret_key: &T,
        public_key: &Self::PublicKey,
    ) -> Self::PublicKey;
}

pub struct FpParams<const P: T> {
    pub p: T,
    pub g: Fp<P>,
    pub q: T,
}
impl<const P: T> ParamsForDiffieHellman for FpParams<P> {
    type G = Fp<P>;
    fn get_g(&self) -> Self::G {
        self.g
    }
    fn get_q(&self) -> T {
        self.q
    }
}
impl<const P: T> DiffieHellman for Fp<P> {
    /// (P, G, Q)
    type Params = FpParams<P>;
    // type SecretKey = T;
    type PublicKey = Self;

    fn generate_secret_key(params: &Self::Params, random_value: T) -> T {
        assert_eq!(params.p, P);
        // let q = Self::order_of_group_of_g(g);
        1 + ((random_value - 1) % (params.q - 1))
    }

    fn compute_public_key(params: &Self::Params, secret_key: &T) -> Self::PublicKey {
        assert_eq!(params.p, P);
        params.g.pow(*secret_key)
    }

    fn compute_shared_secret(
        params: &Self::Params,
        secret_key: &T,
        public_key: &Self::PublicKey,
    ) -> Self::PublicKey {
        assert_eq!(params.p, P);
        public_key.pow(*secret_key)
    }
}

pub struct FpkParams<const P: T, const K: T> {
    pub p: T,
    pub k: T,
    pub g: Fpk<P, K>,
    pub q: T,
}
impl<const P: T, const K: T> ParamsForDiffieHellman for FpkParams<P, K> {
    type G = Fpk<P, K>;
    fn get_g(&self) -> Self::G {
        self.g.clone()
    }
    fn get_q(&self) -> T {
        self.q
    }
}
impl<const P: T, const K: T> DiffieHellman for Fpk<P, K> {
    /// (P, K, G, Q)
    type Params = FpkParams<P, K>;
    // type SecretKey = T;
    type PublicKey = Self;

    fn generate_secret_key(params: &Self::Params, random_value: T) -> T {
        assert_eq!(params.p, P);
        assert_eq!(params.k, K);
        // let q = Self::order_of_group_of_g(g);
        1 + ((random_value - 1) % (params.q - 1))
    }

    fn compute_public_key(params: &Self::Params, secret_key: &T) -> Self::PublicKey {
        assert_eq!(params.p, P);
        assert_eq!(params.k, K);
        params.g.clone().pow(*secret_key)
    }

    fn compute_shared_secret(
        params: &Self::Params,
        secret_key: &T,
        public_key: &Self::PublicKey,
    ) -> Self::PublicKey {
        assert_eq!(params.p, P);
        assert_eq!(params.k, K);
        public_key.clone().pow(*secret_key)
    }
}

pub struct F2mParams<const M: T> {
    pub m: T,
    pub g: F2m<M>,
    pub q: T,
}
impl<const M: T> ParamsForDiffieHellman for F2mParams<M> {
    type G = F2m<M>;
    fn get_g(&self) -> Self::G {
        self.g.clone()
    }
    fn get_q(&self) -> T {
        self.q
    }
}

impl<const M: T> DiffieHellman for F2m<M> {
    /// (M, G, Q)
    type Params = F2mParams<M>;
    // type SecretKey = T;
    type PublicKey = Self;

    fn generate_secret_key(params: &Self::Params, random_value: T) -> T {
        assert_eq!(params.m, M);
        // let q = Self::order_of_group_of_g(&g);
        1 + ((random_value - 1) % (params.q - 1))
    }

    fn compute_public_key(params: &Self::Params, secret_key: &T) -> Self::PublicKey {
        assert_eq!(params.m, M);
        params.g.clone().pow(*secret_key)
    }

    fn compute_shared_secret(
        params: &Self::Params,
        secret_key: &T,
        public_key: &Self::PublicKey,
    ) -> Self::PublicKey {
        assert_eq!(params.m, M);
        public_key.clone().pow(*secret_key)
    }
}

pub struct EcPointParams<Y>
where
    Y: Field + Pow,
    Ec<Y>: EcCalculations<Y>,
    EcPoint<Y>: Add<Output = EcPoint<Y>>,
{
    pub a: Y,
    pub b: Y,
    pub g: EcPoint<Y>,
    pub q: T,
}
impl<Y> ParamsForDiffieHellman for EcPointParams<Y>
where
    Y: Field + Pow + ToJsonSchnorr,
    Ec<Y>: EcCalculations<Y>,
    EcPoint<Y>: Add<Output = EcPoint<Y>> + Neg<Output = EcPoint<Y>> + Inverse,
{
    type G = EcPoint<Y>;
    fn get_g(&self) -> Self::G {
        self.g.clone()
    }
    fn get_q(&self) -> T {
        self.q
    }
}
impl<Y> DiffieHellman for EcPoint<Y>
where
    Y: Field + Pow + ToJsonSchnorr,
    Ec<Y>: EcCalculations<Y>,
    EcPoint<Y>: Add<Output = EcPoint<Y>> + Neg<Output = EcPoint<Y>>,
{
    /// (A, B, (G.x, G.y), Q)
    type Params = EcPointParams<Y>;
    // type SecretKey = T;
    type PublicKey = Self;

    fn generate_secret_key(params: &Self::Params, random_value: T) -> T {
        let EcPointParams { a, b, g, q } = params;
        let _ec: Ec<Y> = Ec::new(a.clone(), b.clone());
        let temp = EcPoint::new(a.clone(), b.clone(), _ec).unwrap();
        assert!(EcPoint::match_ec(g, &temp));
        // let q = Self::order_of_group_of_g(&_g);

        1 + ((random_value - 1) % (q - 1))
    }

    fn compute_public_key(params: &Self::Params, secret_key: &T) -> Self::PublicKey {
        let EcPointParams { a, b, g, q: _ } = params;
        let _ec: Ec<Y> = Ec::new(a.clone(), b.clone());
        let temp = EcPoint::new(a.clone(), b.clone(), _ec).unwrap();
        assert!(EcPoint::match_ec(g, &temp));

        g.clone().pow(*secret_key)
    }

    fn compute_shared_secret(
        params: &Self::Params,
        secret_key: &T,
        public_key: &Self::PublicKey,
    ) -> Self::PublicKey {
        let _ec: Ec<Y> = Ec::new(params.a.clone(), params.b.clone());
        let temp = EcPoint::new(params.a.clone(), params.b.clone(), _ec).unwrap();
        assert!(EcPoint::match_ec(&params.g, &temp));
        public_key.clone().pow(*secret_key)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use lab2::{
        // elliptic_curve::{Ec, EcPoint},
        f2m::{F2m, bit::Bits8},
        fp::Fp,
        fpk::Fpk,
        polynomials::Polynomial,
    };

    fn assert_dh_exchange<D: DiffieHellman>(
        params: &D::Params,
        alice_rand: T,
        bob_rand: T,
    ) -> (
        (T, D::PublicKey, D::PublicKey),
        (T, D::PublicKey, D::PublicKey),
    )
    where
        D::PublicKey: std::fmt::Debug + PartialEq,
        // T: std::fmt::Debug,
    {
        // 1. Alice generates her keys
        let alice_priv = D::generate_secret_key(params, alice_rand);
        let alice_pub = D::compute_public_key(params, &alice_priv);

        // 2. Bob generates his keys
        let bob_priv = D::generate_secret_key(params, bob_rand);
        let bob_pub = D::compute_public_key(params, &bob_priv);

        // 3. Both compute the shared secret using the other's public key
        let alice_shared = D::compute_shared_secret(params, &alice_priv, &bob_pub);
        let bob_shared = D::compute_shared_secret(params, &bob_priv, &alice_pub);

        // 4. Assertions
        assert_eq!(
            alice_shared, bob_shared,
            "Alice and Bob computed different shared secrets!"
        );

        assert_ne!(
            alice_shared, alice_pub,
            "Alice's shared secret should not be the same as a public key"
        );
        assert_ne!(
            bob_shared, bob_pub,
            "Bob's Shared secret should not be the same as a public key"
        );

        (
            (alice_priv, alice_pub, alice_shared),
            (bob_priv, bob_pub, bob_shared),
        )
    }

    #[test]
    fn test_dh_fp_prime_field() {
        const P: T = 23;
        let g = Fp::<P>::new(5);
        let q = 22;

        let params = FpParams { p: P, g, q };

        let (a_res, b_res) = assert_dh_exchange::<Fp<P>>(&params, 6, 15);

        // Example taken from wikipedia
        assert_eq!(a_res.0, 6);
        assert_eq!(a_res.1, Fp::<P>::new(8));
        assert_eq!(a_res.2, Fp::<P>::new(2));
        assert_eq!(b_res.0, 15);
        assert_eq!(b_res.1, Fp::<P>::new(19));
        assert_eq!(b_res.2, Fp::<P>::new(2));
    }

    #[test]
    fn test_dh_fpk() {
        const P: T = 23;
        const K: T = 11;
        let poly = Polynomial::<Fp<P>>::new_from_slice(&[5, 4, 6, 7, 1]);
        let modulo = Polynomial::<Fp<P>>::new_from_slice(&[5, 4, 0, 7, 1, 4, 5, 4, 0, 7, 1, 4]);
        let g = Fpk::<P, K>::new(poly, modulo);
        let q = 100;

        let params = FpkParams { p: P, k: K, g, q };

        _ = assert_dh_exchange::<Fpk<P, K>>(&params, 4, 7);
    }

    #[test]
    fn test_dh_f2m() {
        const M: T = 10;
        let g = F2m::<M>::new_from_slice(
            &[Bits8(0b01101000), Bits8(0b10)],
            &[Bits8(0b01101000), Bits8(0b110)],
        );
        let q = 100;

        let params = F2mParams { m: M, g, q };

        _ = assert_dh_exchange::<F2m<M>>(&params, 4, 7);
    }

    #[test]
    fn test_dh_elliptic_curve() {
        const P: T = 17;
        type Y = Fp<P>;

        let a = Y::from(2);
        let b = Y::from(2);

        let gx = Y::from(5);
        let gy = Y::from(1);
        let g = EcPoint::new(gx, gy, Ec::new(a, b)).unwrap();

        let params = EcPointParams { a, b, g, q: 15 };

        assert_dh_exchange::<EcPoint<Y>>(&params, 3, 7);
    }
}
