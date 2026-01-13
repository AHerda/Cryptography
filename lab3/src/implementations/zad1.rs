use lab2::{
    f2m::{F2m, bit::Bits8},
    polynomials::Polynomial,
    traits::Pow,
};

pub static IRREDUCABLE: [Bits8; 17] = [
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

pub fn ghash(mut h: [u8; 16], a: &[u8], c: &[u8]) -> Vec<u8> {
    h.reverse();
    let bit_len_a = a.len() as u64 * 8;
    let bit_len_c = c.len() as u64 * 8;
    let h: F2m<128> = F2m::new_from_slice(&h.map(|b| Bits8(b.reverse_bits())), &IRREDUCABLE);
    let a: Vec<Bits8> = a.iter().map(|b| Bits8(*b)).collect();
    let c: Vec<Bits8> = c.iter().map(|b| Bits8(*b)).collect();

    let padded_a = pad_with_zeros(a);
    // let m = padded_a.len() as u64 / 16;
    let padded_c = pad_with_zeros(c);
    // let n = padded_c.len() as u64 / 16;

    // let mut s = Vec::new();
    // s.extend_from_slice(&padded_a);
    // s.extend_from_slice(&padded_c);
    // s.extend_from_slice(&bit_len_a.to_be_bytes().map(|b| Bits8(b)));
    // s.extend_from_slice(&bit_len_c.to_be_bytes().map(|b| Bits8(b)));
    let len_block = (((bit_len_a as u128) << 64) + bit_len_c as u128)
        .to_be_bytes()
        .map(|b| Bits8(b));
    let s = padded_a
        .chunks(16)
        .chain(padded_c.chunks(16))
        .chain(len_block.chunks(16));
    // let s: Vec<F2m<128>> = s
    //     .chunks(16)
    //     .map(|w| {
    //         let mut w2 = w.to_vec().clone();
    //         w2.reverse();
    //         F2m::new_from_slice(&w2, &IRREDUCABLE)
    //     })
    //     .collect();

    let mut x = h.zero();

    for block in s {
        let vec: Vec<Bits8> = block
            .iter()
            .map(|b| Bits8(b.0.reverse_bits()))
            .rev()
            .collect();
        let elem = F2m::new_from_slice(&vec, &IRREDUCABLE);
        x = (x + elem) * h.clone();
    }

    x.to_vec()
    // .iter()
    // .rev()
    // .map(|b| b.reverse_bits())
    // .collect()
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

    // #[test]
    // fn test_ghash_zeroed_inputs() {
    //     let h = b"";
    //     let a = b"";
    //     let c = b"";

    //     let tag1 = ghash(h.clone(), a.clone(), c.clone());
    //     let tag2 = ghash(h, a, c);

    //     assert!(tag1.degree().is_none());
    //     assert!(tag2.degree().is_none());
    // }

    #[test]
    fn test_ghash_no_blocks() {
        let h = [
            0x66, 0xe9, 0x4b, 0xd4, 0xef, 0x8a, 0x2c, 0x3b, 0x88, 0x4c, 0xfa, 0x59, 0xca, 0x34,
            0x2b, 0x2e,
        ];
        let a = b"";
        let c = b"";

        let tag1 = ghash(h.clone(), a, c);
        let tag2 = ghash(h, a, c);

        assert!(tag1.iter().all(|&b| b == 0));
        assert!(tag2.iter().all(|&b| b == 0));
    }

    #[test]
    fn test_ghash_single_block() {
        let h = [
            0x66, 0xe9, 0x4b, 0xd4, 0xef, 0x8a, 0x2c, 0x3b, 0x88, 0x4c, 0xfa, 0x59, 0xca, 0x34,
            0x2b, 0x2e,
        ];
        let a = [
            0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E,
            0x0F, 0x10,
        ];
        let c = b"";

        let tag1 = ghash(h, &a, c);
        let tag2 = ghash(h, &a, c);

        assert!(tag1.iter().any(|&b| b != 0));
        assert!(tag2.iter().any(|&b| b != 0));
        assert_eq!(tag1, tag2);
    }

    #[test]
    fn test_ghash_multiple_blocks() {
        let h = [
            0x66, 0xe9, 0x4b, 0xd4, 0xef, 0x8a, 0x2c, 0x3b, 0x88, 0x4c, 0xfa, 0x59, 0xca, 0x34,
            0x2b, 0x2e,
        ];
        let a = b"Hello, World! :)    ";
        let c = b"1234567890";

        let tag1 = ghash(h, a, c);
        let tag2 = ghash(h, a, c);

        assert!(tag1.iter().any(|&b| b != 0));
        assert!(tag2.iter().any(|&b| b != 0));
        assert_eq!(tag1, tag2);
    }

    #[test]
    fn test_54_byte_block() {
        let h = 0x73A23D80121DE2D5A850253FCF43120E_u128.to_be_bytes();
        let mut a = [
            0xD6, 0x09, 0xB1, 0xF0, 0x56, 0x63, 0x7A, 0x0D, 0x46, 0xDF, 0x99, 0x8D, 0x88, 0xE5,
            0x22, 0x2A, 0xB2, 0xC2, 0x84, 0x65, 0x12, 0x15, 0x35, 0x24, 0xC0, 0x89, 0x5E, 0x81,
            0x08, 0x00, 0x0F, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1A,
            0x1B, 0x1C, 0x1D, 0x1E, 0x1F, 0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28,
            0x29, 0x2A, 0x2B, 0x2C, 0x2D, 0x2E, 0x2F, 0x30, 0x31, 0x32, 0x33, 0x34, 0x00, 0x01,
        ];
        // a.reverse();
        let c = b"";
        let tag = ghash(h, &a, c);
        let expected = 0x1BDA7DB505D8A165264986A703A6920D_u128.to_be_bytes();
        println!("{:#02x?}", tag);
        println!("{:#02x?}", expected);
        assert_eq!(tag, expected);
    }
}
