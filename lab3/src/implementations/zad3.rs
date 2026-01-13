use std::{
    collections::HashSet,
    ops::{Add, Mul},
};

use lab2::{
    T,
    elliptic_curve::{Ec, EcPoint},
    f2m::F2m,
    fp::Fp,
    fpk::Fpk,
    traits::{EcCalculations, Field, Pow},
};

pub trait DiffieHellman {
    type G: Mul + PartialEq + Pow;
    type Params;
    type SecretKey;
    type PublicKey;

    fn order_of_group_of_g(g: &Self::G) -> T {
        let mut temp = g.clone();
        let mut seen = vec![];
        while !seen.contains(&temp) && temp != temp.one() {
            seen.push(temp.clone());
            temp = temp * g.clone();
        }
        seen.len() + 1
    }
    fn generate_secret_key(params: &Self::Params, random_value: T) -> Self::SecretKey;
    fn compute_public_key(params: &Self::Params, secret_key: &Self::SecretKey) -> Self::PublicKey;
    fn compute_shared_secret(
        params: &Self::Params,
        secret_key: &Self::SecretKey,
        public_key: &Self::PublicKey,
    ) -> Self::PublicKey;
}

impl<const P: T> DiffieHellman for Fp<P> {
    type G = Self;
    /// (P, G, Q)
    type Params = (T, Self::G, T);
    type SecretKey = T;
    type PublicKey = Self;

    fn generate_secret_key(params: &Self::Params, random_value: T) -> Self::SecretKey {
        let (p, g, q) = params;
        assert_eq!(*p, P);
        // let q = Self::order_of_group_of_g(g);
        1 + ((random_value - 1) % (q - 1))
    }

    fn compute_public_key(params: &Self::Params, secret_key: &Self::SecretKey) -> Self::PublicKey {
        let (p, g, _) = params;
        assert_eq!(*p, P);
        g.pow(*secret_key)
    }

    fn compute_shared_secret(
        params: &Self::Params,
        secret_key: &Self::SecretKey,
        public_key: &Self::PublicKey,
    ) -> Self::PublicKey {
        let p = params.0;
        assert_eq!(p, P);
        public_key.pow(*secret_key)
    }
}

impl<const P: T, const K: T> DiffieHellman for Fpk<P, K> {
    type G = Self;
    /// (P, K, G, Q)
    type Params = (T, T, Self::G, T);
    type SecretKey = T;
    type PublicKey = Self;

    fn generate_secret_key(params: &Self::Params, random_value: T) -> Self::SecretKey {
        let (p, k, g, q) = params;
        assert_eq!(*p, P);
        assert_eq!(*k, K);
        // let q = Self::order_of_group_of_g(g);
        1 + ((random_value - 1) % (q - 1))
    }

    fn compute_public_key(params: &Self::Params, secret_key: &Self::SecretKey) -> Self::PublicKey {
        let (p, k, g, _) = params;
        assert_eq!(*p, P);
        assert_eq!(*k, K);
        g.clone().pow(*secret_key)
    }

    fn compute_shared_secret(
        params: &Self::Params,
        secret_key: &Self::SecretKey,
        public_key: &Self::PublicKey,
    ) -> Self::PublicKey {
        let (p, k, _, _) = params;
        assert_eq!(*p, P);
        assert_eq!(*k, K);
        public_key.clone().pow(*secret_key)
    }
}

impl<const M: T> DiffieHellman for F2m<M> {
    type G = Self;
    /// (M, G, Q)
    type Params = (T, Self::G, T);
    type SecretKey = T;
    type PublicKey = Self;

    fn generate_secret_key(params: &Self::Params, random_value: T) -> Self::SecretKey {
        let (m, g, q) = params;
        assert_eq!(*m, M);
        // let q = Self::order_of_group_of_g(&g);
        1 + ((random_value - 1) % (q - 1))
    }

    fn compute_public_key(params: &Self::Params, secret_key: &Self::SecretKey) -> Self::PublicKey {
        let (m, g, _) = params;
        assert_eq!(*m, M);
        g.clone().pow(*secret_key)
    }

    fn compute_shared_secret(
        params: &Self::Params,
        secret_key: &Self::SecretKey,
        public_key: &Self::PublicKey,
    ) -> Self::PublicKey {
        let m = params.0;
        assert_eq!(m, M);
        public_key.clone().pow(*secret_key)
    }
}

impl<Y> DiffieHellman for EcPoint<Y>
where
    Y: Field + Pow,
    Ec<Y>: EcCalculations<Y>,
    EcPoint<Y>: Add<Output = Self>,
{
    type G = Self;
    /// (A, B, (G.x, G.y), Q)
    type Params = (Y, Y, (Y, Y), T);
    type SecretKey = T;
    type PublicKey = Self;

    fn generate_secret_key(params: &Self::Params, random_value: T) -> Self::SecretKey {
        let (a, b, g, q) = params;
        let _ec: Ec<Y> = Ec::new(a.clone(), b.clone());
        let _g = Self::new(g.0.clone(), g.1.clone(), _ec).unwrap();
        // let q = Self::order_of_group_of_g(&_g);

        1 + ((random_value - 1) % (q - 1))
    }

    fn compute_public_key(params: &Self::Params, secret_key: &Self::SecretKey) -> Self::PublicKey {
        let (a, b, g, _) = params;
        let ec: Ec<Y> = Ec::new(a.clone(), b.clone());
        let g = Self::new(g.0.clone(), g.1.clone(), ec).unwrap();

        g.clone().pow(*secret_key)
    }

    fn compute_shared_secret(
        _: &Self::Params,
        secret_key: &Self::SecretKey,
        public_key: &Self::PublicKey,
    ) -> Self::PublicKey {
        public_key.clone().pow(*secret_key)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use lab2::{
        elliptic_curve::{Ec, EcPoint},
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
        (D::SecretKey, D::PublicKey, D::PublicKey),
        (D::SecretKey, D::PublicKey, D::PublicKey),
    )
    where
        D::PublicKey: std::fmt::Debug + PartialEq,
        D::SecretKey: std::fmt::Debug,
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

        let params = (P, g, q);

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
    fn test_dh_fpk_extension_field() {
        const P: T = 23;
        const K: T = 11;
        let poly = Polynomial::<Fp<P>>::new_from_slice(&[5, 4, 6, 7, 1]);
        let modulo = Polynomial::<Fp<P>>::new_from_slice(&[5, 4, 0, 7, 1, 4, 5, 4, 0, 7, 1, 4]);
        let g = Fpk::<P, K>::new(poly, modulo);
        let q = 100;

        let params = (P, K, g, q);

        _ = assert_dh_exchange::<Fpk<P, K>>(&params, 4, 7);
    }

    #[test]
    fn test_dh_f2m_binary_field() {
        const M: T = 10;
        let g = F2m::<M>::new_from_slice(
            &[Bits8(0b01101000), Bits8(0b10)],
            &[Bits8(0b01101000), Bits8(0b110)],
        );
        let q = 100;

        let params = (M, g, q);

        _ = assert_dh_exchange::<F2m<M>>(&params, 4, 7);
    }

    // #[test]
    // fn test_dh_elliptic_curve() {
    //     // We need a small curve defined over a small Finite Field to test logic
    //     // without exhausting memory in `order_of_group_of_g`.

    //     // 1. Define Field Y = Fp<17>
    //     const P: T = 17;
    //     type Y = Fp<P>;

    //     // 2. Define Curve: y^2 = x^3 + 2x + 2 (mod 17)
    //     // a = 2, b = 2
    //     let a = Y::from(2);
    //     let b = Y::from(2);

    //     // 3. Define Generator Point G = (5, 1)
    //     // Check: 1^2 = 1. 5^3 + 2*5 + 2 = 125 + 10 + 2 = 137. 137 % 17 = 1. Matches.
    //     let gx = Y::from(5);
    //     let gy = Y::from(1);

    //     // Params structure for EcPoint is (a, b, (gx, gy))
    //     let params = (a, b, (gx, gy));

    //     // Note: The EcPoint implementation in your code reconstructs the curve
    //     // inside `generate_secret_key`, so we just pass the raw field elements.

    //     assert_dh_exchange::<EcPoint<Y>>(&params, 3, 7);
    // }
}
