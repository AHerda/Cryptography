pub fn md5(input: &str) -> u128 {
    0x0
}


#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    fn test_md5() {
        assert_eq!(md5(""), 0xd41d8cd98f00b204e9800998ecf8427e);
        assert_eq!(md5("a"), 0x0cc175b9c0f1b6a831c399e269772661);
        assert_eq!(md5("abc"), 0x900150983cd24fb0d6963f7d28e17f72);
        assert_eq!(md5("message digest"), 0xf96b697d7cb7938d525a2f31aaf161d0);
        assert_eq!(md5("abcdefghijklmnopqrstuvwxyz"), 0xc3fcd3d76192e4007dfb496cca67e13b);
        assert_eq!(md5("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789"), 0xd174ab98d277d9f5a5611c2c9f419d9f);
        assert_eq!(md5("12345678901234567890123456789012345678901234567890123456789012345678901234567890"), 0x57edf4a22be3c955ac49da2e2107b67a);
    }
}