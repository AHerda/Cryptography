pub fn f(x: u32, y: u32, z: u32) -> u32 {
    (x & y) | (!x & z)
}

pub fn g(x: u32, y: u32, z: u32) -> u32 {
    (x & z) | (y & !z)
}

pub fn h(x: u32, y: u32, z: u32) -> u32 {
    x ^ y ^ z
}

pub fn i(x: u32, y: u32, z: u32) -> u32 {
    y ^ (x | !z)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_f() {
        assert_eq!(f(0b1100, 0b1010, 0b1001), 0b1001);
    }

    #[test]
    fn test_g() {
        assert_eq!(g(0b1100, 0b1010, 0b1001), 0b1010);
    }

    #[test]
    fn test_h() {
        assert_eq!(h(0b1100, 0b1110, 0b1000), 0b1010);
    }

    #[test]
    fn test_i() {
        // Note: In Rust, the bitwise NOT operator `!` inverts all bits, so for a 32-bit integer,
        // !0b1001 results in 0xFFFFFFF6. Therefore, the expected is just manipulation of
        // last four bits as y can only have 1s in those positions. The rest will be 1s due to the NOT operation.
        assert_eq!(i(0b1100, 0b1010, 0b1001), 0xFFFFFFF0 | 0b0100);
    }
}