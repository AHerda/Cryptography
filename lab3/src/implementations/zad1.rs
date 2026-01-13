use lab2::{
    f2m::{F2m, bit::Bits8},
    polynomials::Polynomial,
    traits::Pow,
};

static IRREDUCABLE: [Bits8; 17] = [
    Bits8(0b10000111),
    Bits8(0),
    Bits8(0),
    Bits8(0),
    Bits8(0),
    Bits8(0),
    Bits8(0),
    Bits8(0),
    Bits8(0),
    Bits8(0),
    Bits8(0),
    Bits8(0),
    Bits8(0),
    Bits8(0),
    Bits8(0),
    Bits8(0),
    Bits8(1),
];

pub fn ghash(h: F2m<128>, a: Polynomial<Bits8>, c: Polynomial<Bits8>) -> F2m<128> {
    let modulo = h.get_modulo().coefficients();
    assert_eq!(modulo, IRREDUCABLE);
    let zero = h.zero();

    let padded_a = pad_with_zeros(a.coefficients());
    let m = padded_a.len() as u64;
    let padded_c = pad_with_zeros(c.coefficients());
    let n = padded_c.len() as u64;

    let mut s = Vec::new();
    s.extend_from_slice(&padded_a);
    s.extend_from_slice(&padded_c);
    s.extend_from_slice(&a.degree().unwrap_or(0).to_le_bytes().map(|b| Bits8(b)));
    s.extend_from_slice(&c.degree().unwrap_or(0).to_le_bytes().map(|b| Bits8(b)));
    let s: Vec<F2m<128>> = s
        .windows(16)
        .map(|w| F2m::new_from_slice(w, &modulo))
        .collect();

    let mut x: Vec<F2m<128>> = Vec::with_capacity((m + n) as usize + 2);
    x.push(zero);

    for i in 0..m + n + 1 {
        x.push((x[i as usize].clone() + s[i as usize].clone()) * h.clone());
    }

    x[(m + n) as usize + 1].clone()
}

fn pad_with_zeros(v: Vec<Bits8>) -> Vec<Bits8> {
    let mut padded = v.clone();
    while padded.len() % 16 != 0 {
        padded.push(Bits8(0));
    }
    padded
}

#[cfg(test)]
mod tests {
    use lab2::polynomials::Polynomial;

    use super::*;

    #[test]
    fn test_padding_with_zeros() {
        let mut expected = vec![Bits8(0); 16];
        let mut v = vec![];
        for i in 0..10 {
            expected[i] = Bits8(1);
            v.push(Bits8(1));

            let padded = pad_with_zeros(v.clone());

            assert_eq!(padded, expected);
            assert_eq!(Polynomial::new(padded), Polynomial::new(v.clone()));
        }
    }

    #[test]
    fn test_ghash_zeroed_inputs() {
        let h_bytes = b"";
        let a_bytes = b"";
        let c_bytes = b"";

        let h: F2m<128> = F2m::new_from_slice(&h_bytes.map(|b| Bits8(b)), &IRREDUCABLE);
        let a: Polynomial<Bits8> = Polynomial::new(a_bytes.map(|b| Bits8(b)).to_vec());
        let c: Polynomial<Bits8> = Polynomial::new(c_bytes.map(|b| Bits8(b)).to_vec());

        let tag1 = ghash(h.clone(), a.clone(), c.clone());
        let tag2 = ghash(h, a, c);

        assert!(tag1.degree().is_none());
        assert!(tag2.degree().is_none());
    }

    #[test]
    fn test_ghash_no_blocks() {
        let h_bytes = [
            0x66, 0xe9, 0x4b, 0xd4, 0xef, 0x8a, 0x2c, 0x3b, 0x88, 0x4c, 0xfa, 0x59, 0xca, 0x34,
            0x2b, 0x2e,
        ];
        let a_bytes = b"";
        let c_bytes = b"";

        let h: F2m<128> = F2m::new_from_slice(&h_bytes.map(|b| Bits8(b)), &IRREDUCABLE);
        let a: Polynomial<Bits8> = Polynomial::new(a_bytes.map(|b| Bits8(b)).to_vec());
        let c: Polynomial<Bits8> = Polynomial::new(c_bytes.map(|b| Bits8(b)).to_vec());

        let tag1 = ghash(h.clone(), a.clone(), c.clone());
        let tag2 = ghash(h, a, c);

        assert!(tag1.degree().is_none());
        assert!(tag2.degree().is_none());
    }

    #[test]
    fn test_ghash_single_block() {
        let h_bytes = [
            0x66, 0xe9, 0x4b, 0xd4, 0xef, 0x8a, 0x2c, 0x3b, 0x88, 0x4c, 0xfa, 0x59, 0xca, 0x34,
            0x2b, 0x2e,
        ];
        let a_bytes = [
            0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E,
            0x0F, 0x10,
        ];
        let c_bytes = b"";

        let h: F2m<128> = F2m::new_from_slice(&h_bytes.map(|b| Bits8(b)), &IRREDUCABLE);
        let a: Polynomial<Bits8> = Polynomial::new(a_bytes.map(|b| Bits8(b)).to_vec());
        let c: Polynomial<Bits8> = Polynomial::new(c_bytes.map(|b| Bits8(b)).to_vec());

        let tag1 = ghash(h.clone(), a.clone(), c.clone());
        let tag2 = ghash(h, a, c);

        assert!(tag1.degree().is_some());
        assert!(tag2.degree().is_some());
        assert_eq!(tag1, tag2);
    }

    #[test]
    fn test_ghash_multiple_blocks() {
        let h_bytes = [
            0x66, 0xe9, 0x4b, 0xd4, 0xef, 0x8a, 0x2c, 0x3b, 0x88, 0x4c, 0xfa, 0x59, 0xca, 0x34,
            0x2b, 0x2e,
        ];
        let a_bytes = b"Hello, World! :)    ";
        let c_bytes = b"1234567890";

        let h: F2m<128> = F2m::new_from_slice(&h_bytes.map(|b| Bits8(b)), &IRREDUCABLE);
        let a: Polynomial<Bits8> = Polynomial::new(a_bytes.map(|b| Bits8(b)).to_vec());
        let c: Polynomial<Bits8> = Polynomial::new(c_bytes.map(|b| Bits8(b)).to_vec());

        let tag1 = ghash(h.clone(), a.clone(), c.clone());
        let tag2 = ghash(h, a, c);

        assert!(tag1.degree().is_some());
        assert!(tag2.degree().is_some());
        assert_eq!(tag1, tag2);
    }
}
