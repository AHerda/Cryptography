use lab2::{
    T,
    elliptic_curve::{Ec, EcPoint},
    f2m::{F2m, bit::Bits8},
    fp::Fp,
    fpk::Fpk,
    traits::Field,
};
use sha2::{Digest, Sha256};

pub trait ToJsonSchnorr {
    fn encode(&self) -> String;
}

impl<const P: T> ToJsonSchnorr for Fp<P> {
    fn encode(&self) -> String {
        let mut p_len = format!("{:x}", P).len();
        p_len += p_len % 2;
        let number = format!("{:X}", self.get());
        let mut result: String = (0..=p_len - number.len())
            .map(|i| if i == 0 { '"' } else { '0' })
            .collect();
        result.push_str(&number);
        result.push_str(r#"""#);
        result
    }
}

impl<const P: T, const K: T> ToJsonSchnorr for Fpk<P, K> {
    fn encode(&self) -> String {
        let mut result = "[".to_string();
        let mut coef: Vec<_> = self.coefficients().iter().map(|fp| fp.encode()).collect();

        while coef.len() < K {
            coef.push(r#""00""#.to_string());
        }
        for c in coef {
            result.push_str(&c);
        }

        result = result.replace(r#""""#, r#"",""#);
        result.push(']');
        result
    }
}

impl<const M: T> ToJsonSchnorr for F2m<M> {
    fn encode(&self) -> String {
        let mut bytes: Vec<_> = self
            .coefficients()
            .iter()
            .map(|byte| format!("{:02X}", byte.0))
            .collect();
        let mut len = M % 8;
        if len != 0 {
            len = (M + 8 - len) / 8;
        } else {
            len = M;
        }

        while bytes.len() < len {
            bytes.push("00".to_string());
        }
        bytes.push("\"".to_string());
        bytes.reverse();
        bytes.push("\"".to_string());
        bytes.concat()
    }
}

impl<T: ToJsonSchnorr + Field> ToJsonSchnorr for EcPoint<T> {
    fn encode(&self) -> String {
        let (x, y);
        match self {
            EcPoint::Point {
                x: x_p,
                y: y_p,
                ec: _,
            } => (x, y) = (x_p, y_p),
            EcPoint::Infinity => return "infinty".to_string(),
        }
        let mut result = r#"{"x":"#.to_string();
        result.push_str(&x.encode());
        result.push_str(r#","y":"#);
        result.push_str(&y.encode());
        result.push('}');
        result
    }
}

pub fn ss(r: impl ToJsonSchnorr, m: &str) -> String {
    let mut input = r.encode();
    input.extend(m.chars());
    Sha256::digest(input)
        .iter()
        .map(|b| format!("{:02x}", b))
        .collect::<String>()
}

#[cfg(test)]
mod tests {
    use lab2::polynomials::Polynomial;

    use super::*;

    #[test]
    fn test_fp() {
        let fp: Fp<65537> = Fp::new(17);
        assert_eq!(fp.encode(), r#""000011""#);
    }

    #[test]
    fn test_fpk() {
        let modulo: Polynomial<Fp<17>> = Polynomial::new_from_slice(&[6, 4, 5, 1]);
        let poly: Polynomial<Fp<17>> = Polynomial::new_from_slice(&[16, 0, 3]);
        let fpk = Fpk::<17, 3>::new(poly, modulo);

        assert_eq!(fpk.encode(), r#"["10","00","03"]"#);
    }

    #[test]
    fn test_f2m() {
        let f2m = F2m::<33>::new_from_slice(
            &[Bits8(0b1101)],
            &[Bits8(6), Bits8(4), Bits8(5), Bits8(255), Bits8(2)],
        );

        assert_eq!(f2m.encode(), r#""000000000D""#);
    }

    #[test]
    fn test_ec_over_fp() {
        let x = Fp::<17>::new(3);
        let y = Fp::<17>::new(5);
        let ec = Ec::new(x.clone(), y.clone());
        let point = EcPoint::new(x, y, ec).unwrap();

        assert_eq!(point.encode(), r#"{"x":"03","y":"05"}"#)
    }

    #[test]
    fn test_schnorr_signature() {
        let m = "Alice";
        let value = ss(Fp::<65537>::new(17), m);
        let expected = "faf463d7d5cf8b6ca0383bcb37b373b71c5ad7e9f0618e0747400fc1ee571830";
        assert_eq!(value, expected);
    }
}
