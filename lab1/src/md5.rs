use super::{bit_functions::*, consts, state::State};

pub struct Md5(State);

impl Md5 {
    pub fn new_with_state(input: impl AsRef<[u8]>, mut state: State) -> Self {
        Self::padding(input)
            .chunks(4)
            .map(|chunk_4| u32::from_le_bytes(chunk_4.try_into().unwrap()))
            .collect::<Vec<_>>()
            .chunks(16)
            .for_each(|block| {
                let mut temp_state = state;

                Self::rounds(&mut temp_state, &block);

                state += temp_state;
            });

        Self(state)
    }

    pub fn new_with_state_raw_block(input: &[u32], mut state: State) -> Self {
        let mut temp_state = state;
        Self::rounds(&mut temp_state, &input);
        state += temp_state;

        Self(state)
    }

    pub fn new(input: impl AsRef<[u8]>) -> Self {
        let state = State::new();

        Self::new_with_state(input, state)
    }

    pub fn new_raw_block(input: &[u32]) -> Self {
        let state = State::new();

        Self::new_with_state_raw_block(input, state)
    }

    pub(super) fn padding(input: impl AsRef<[u8]>) -> Vec<u8> {
        let bits = input.as_ref().len() as u64 * 8;
        let mut padding_len = (512 - ((bits + 64) % 512)) / 8;
        if padding_len == 0 {
            padding_len = 64;
        }
        assert_eq!(0, (bits + padding_len * 8 + 64) % 512);

        input
            .as_ref()
            .iter()
            .cloned()
            .chain(std::iter::once(0x80_u8))
            .chain(std::iter::repeat_n(0x00_u8, padding_len as usize - 1))
            .chain(bits.to_le_bytes().into_iter())
            .collect::<Vec<u8>>()
    }

    fn rounds(state: &mut State, block: &[u32]) {
        let mut iter: usize = 0;
        macro_rules! round {
            ($round: expr, $function: ident) => {
                let (mut k, inc) = consts::X_INDEX_START[$round];

                sixteen!($function, a, b, c, d, k, inc, consts::S[$round][0], iter);
                sixteen!($function, d, a, b, c, k, inc, consts::S[$round][1], iter);
                sixteen!($function, c, d, a, b, k, inc, consts::S[$round][2], iter);
                sixteen!($function, b, c, d, a, k, inc, consts::S[$round][3], iter);
                sixteen!($function, a, b, c, d, k, inc, consts::S[$round][0], iter);
                sixteen!($function, d, a, b, c, k, inc, consts::S[$round][1], iter);
                sixteen!($function, c, d, a, b, k, inc, consts::S[$round][2], iter);
                sixteen!($function, b, c, d, a, k, inc, consts::S[$round][3], iter);
                sixteen!($function, a, b, c, d, k, inc, consts::S[$round][0], iter);
                sixteen!($function, d, a, b, c, k, inc, consts::S[$round][1], iter);
                sixteen!($function, c, d, a, b, k, inc, consts::S[$round][2], iter);
                sixteen!($function, b, c, d, a, k, inc, consts::S[$round][3], iter);
                sixteen!($function, a, b, c, d, k, inc, consts::S[$round][0], iter);
                sixteen!($function, d, a, b, c, k, inc, consts::S[$round][1], iter);
                sixteen!($function, c, d, a, b, k, inc, consts::S[$round][2], iter);
                sixteen!($function, b, c, d, a, k, inc, consts::S[$round][3], iter);
            };
        }

        macro_rules! sixteen {
            ($func: ident, $a: ident, $b: ident, $c: ident, $d: ident, $k: expr, $inc: expr, $s: expr, $i: expr) => {
                state.$a = (state
                    .$a
                    .wrapping_add($func(state.$b, state.$c, state.$d))
                    .wrapping_add(block[$k])
                    .wrapping_add(consts::T[$i]))
                .rotate_left($s as u32)
                .wrapping_add(state.$b);
                $i += 1;
                $k = ($k + $inc) % 16;
            };
        }

        round!(0, f);
        round!(1, g);
        round!(2, h);
        round!(3, i);
    }

    pub fn to_str(&self) -> String {
        format!("{:032x}", self.0.get_hash())
    }

    pub fn to_str_be(&self) -> String {
        format!("{:032x}", self.0.get_hash_be())
    }

    pub fn get_hash(&self) -> u128 {
        self.0.get_hash()
    }

    pub fn get_hash_be(&self) -> u128 {
        self.0.get_hash_be()
    }

    pub fn get_state(&self) -> State {
        self.0
    }
}

impl Into<u128> for Md5 {
    fn into(self) -> u128 {
        self.get_hash()
    }
}

impl Into<String> for Md5 {
    fn into(self) -> String {
        self.to_str()
    }
}

impl PartialEq for Md5 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_padding() {
        let mut vec = vec![0; 64];
        vec[0] = 0x80;
        assert_eq!(Md5::padding(""), vec);

        vec[0] = 'a' as u8;
        vec[1] = 0x80;
        vec[64 - 8] = 0x8;
        assert_eq!(Md5::padding("a"), vec);
    }

    #[test]
    fn test_md5() {
        assert_eq!(Md5::new("").get_hash(), 0xd41d8cd98f00b204e9800998ecf8427e);
        assert_eq!(Md5::new("a").get_hash(), 0x0cc175b9c0f1b6a831c399e269772661);
        assert_eq!(
            Md5::new("abc").get_hash(),
            0x900150983cd24fb0d6963f7d28e17f72
        );
        assert_eq!(
            Md5::new("message digest").get_hash(),
            0xf96b697d7cb7938d525a2f31aaf161d0
        );
        assert_eq!(
            Md5::new("abcdefghijklmnopqrstuvwxyz").get_hash(),
            0xc3fcd3d76192e4007dfb496cca67e13b
        );
        assert_eq!(
            Md5::new("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789").get_hash(),
            0xd174ab98d277d9f5a5611c2c9f419d9f
        );
        assert_eq!(
            Md5::new(
                "12345678901234567890123456789012345678901234567890123456789012345678901234567890"
            )
            .get_hash(),
            0x57edf4a22be3c955ac49da2e2107b67a
        );
    }
}
