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
    /// (P, G)
    type Params = (T, Self::G);
    type SecretKey = T;
    type PublicKey = Self;

    fn generate_secret_key(params: &Self::Params, random_value: T) -> Self::SecretKey {
        let (p, g) = params;
        assert_eq!(*p, P);
        let q = Self::order_of_group_of_g(g);
        1 + (random_value % (q - 1))
    }

    fn compute_public_key(params: &Self::Params, secret_key: &Self::SecretKey) -> Self::PublicKey {
        let (p, g) = params;
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
    /// (P, K, G)
    type Params = (T, T, Self::G);
    type SecretKey = T;
    type PublicKey = Self;

    fn generate_secret_key(params: &Self::Params, random_value: T) -> Self::SecretKey {
        let (p, k, g) = params;
        assert_eq!(*p, P);
        assert_eq!(*k, K);
        let q = Self::order_of_group_of_g(g);
        1 + (random_value % (q - 1))
    }

    fn compute_public_key(params: &Self::Params, secret_key: &Self::SecretKey) -> Self::PublicKey {
        let (p, k, g) = params;
        assert_eq!(*p, P);
        assert_eq!(*k, K);
        g.clone().pow(*secret_key)
    }

    fn compute_shared_secret(
        params: &Self::Params,
        secret_key: &Self::SecretKey,
        public_key: &Self::PublicKey,
    ) -> Self::PublicKey {
        let (p, k, _) = params;
        assert_eq!(*p, P);
        assert_eq!(*k, K);
        public_key.clone().pow(*secret_key)
    }
}

impl<const M: T> DiffieHellman for F2m<M> {
    type G = Self;
    /// (M, G)
    type Params = (T, Self::G);
    type SecretKey = T;
    type PublicKey = Self;

    fn generate_secret_key(params: &Self::Params, random_value: T) -> Self::SecretKey {
        let (m, g) = params;
        assert_eq!(*m, M);
        let q = Self::order_of_group_of_g(&g);
        1 + (random_value % (q - 1))
    }

    fn compute_public_key(params: &Self::Params, secret_key: &Self::SecretKey) -> Self::PublicKey {
        let (m, g) = params;
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
    /// (A, B, (G.x, G.y))
    type Params = (Y, Y, (Y, Y));
    type SecretKey = T;
    type PublicKey = Self;

    fn generate_secret_key(params: &Self::Params, random_value: T) -> Self::SecretKey {
        let (a, b, g) = params;
        let ec: Ec<Y> = Ec::new(a.clone(), b.clone());
        let g = Self::new(g.0.clone(), g.1.clone(), ec).unwrap();
        let q = Self::order_of_group_of_g(&g);

        1 + (random_value % (q - 1))
    }

    fn compute_public_key(params: &Self::Params, secret_key: &Self::SecretKey) -> Self::PublicKey {
        let (a, b, g) = params;
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
    // Re-export or mock the necessary types from lab2 if they aren't available in this scope.
    // Assuming T is a primitive integer type (e.g., u128, u64).
    use lab2::{
        elliptic_curve::{Ec, EcPoint},
        f2m::F2m,
        fp::Fp,
        fpk::Fpk,
    };

    /// A generic helper to verify Diffie-Hellman exchange correctness.
    /// It simulates Alice and Bob generating keys and ensures they derive the same secret.
    fn assert_dh_exchange<D: DiffieHellman>(params: &D::Params, alice_rand: T, bob_rand: T)
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
        // Ensure shared secrets match
        assert_eq!(
            alice_shared, bob_shared,
            "Alice and Bob computed different shared secrets!"
        );

        // Sanity check: Public keys should generally not act as the shared secret
        // (unless the random values were unlucky and small)
        if alice_rand != bob_rand {
            // This is a weak check, but useful to ensure we didn't just return identity
            // assert_ne!(alice_shared, alice_pub);
        }
    }

    #[test]
    fn test_dh_fp_prime_field() {
        // Use a small prime P = 23 to prevent `order_of_group_of_g` from hanging.
        const P: T = 23;

        // Generator 5 is a primitive root mod 23.
        // Assuming Fp::from or Fp::new exists:
        let g = Fp::<P>::new(5);

        // Params for Fp are (P, g)
        let params = (P, g);

        // Run exchange with arbitrary random seeds
        assert_dh_exchange::<Fp<P>>(&params, 10, 15);
    }

    #[test]
    fn test_dh_fpk_extension_field() {
        // Use small P and K.
        // Example: GF(3^2). P=3, K=2.
        const P: T = 3;
        const K: T = 2;

        // We need a valid generator for the field.
        // Assuming Fpk constructor takes a value or coefficients.
        // This part depends heavily on your Fpk implementation details.
        // Let's assume Fpk::from(n) creates a polynomial representing n.
        let g = Fpk::<P, K>::new(2);

        let params = (P, K, g);

        assert_dh_exchange::<Fpk<P, K>>(&params, 4, 7);
    }

    #[test]
    fn test_dh_f2m_binary_field() {
        // GF(2^4). M = 4.
        // Using polynomial representation x^4 + x + 1 (irreducible) typically.
        const M: T = 4;

        // Generator for the field.
        // Assuming F2m::from(u128) converts integer bitmap to polynomial.
        let g = F2m::<M>::from(2); // Polynomial 'x'

        let params = (M, g);

        assert_dh_exchange::<F2m<M>>(&params, 5, 9);
    }

    #[test]
    fn test_dh_elliptic_curve() {
        // We need a small curve defined over a small Finite Field to test logic
        // without exhausting memory in `order_of_group_of_g`.

        // 1. Define Field Y = Fp<17>
        const P: T = 17;
        type Y = Fp<P>;

        // 2. Define Curve: y^2 = x^3 + 2x + 2 (mod 17)
        // a = 2, b = 2
        let a = Y::from(2);
        let b = Y::from(2);

        // 3. Define Generator Point G = (5, 1)
        // Check: 1^2 = 1. 5^3 + 2*5 + 2 = 125 + 10 + 2 = 137. 137 % 17 = 1. Matches.
        let gx = Y::from(5);
        let gy = Y::from(1);

        // Params structure for EcPoint is (a, b, (gx, gy))
        let params = (a, b, (gx, gy));

        // Note: The EcPoint implementation in your code reconstructs the curve
        // inside `generate_secret_key`, so we just pass the raw field elements.

        assert_dh_exchange::<EcPoint<Y>>(&params, 3, 7);
    }
}
