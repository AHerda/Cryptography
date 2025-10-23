use super::{bit_functions, consts, state::State};

fn padding(input: &str) -> Vec<u8> {
    let bits = input.len() as u64 * 8;
    let mut padding_len = (512 - ((bits + 64) % 512)) / 8;
    if padding_len == 0 {
        padding_len = 512 / 8;
    }
    assert_eq!(0, (bits + padding_len * 8 + 64) % 512);

    let mut result = input.as_bytes().to_vec();
    result.push(0x80);
    result.append(&mut vec![0; padding_len as usize - 1]);
    result.extend(bits.to_le_bytes());
    result
}

pub struct Md5(u128);

impl Md5 {
    pub fn new(input: &str) -> Self {
        let input = padding(input);
        let mut state = State::new();
        let blocks: Vec<Vec<u32>> = input
            .chunks(64)
            .map(|chunk_64| {
                chunk_64
                    .chunks(4)
                    .map(|chunk_4| u32::from_le_bytes(chunk_4.try_into().unwrap()))
                    .collect()
            })
            .collect();

        for block in blocks {
            let mut temp_state = state;

            (0..4).for_each(|r| Self::round(&mut temp_state, &block, r));

            state += temp_state;
        }

        Self(state.get_hash())
    }

        fn round(state: &mut State, block: &[u32], round: usize) {
        let function = match round {
            0 => bit_functions::f,
            1 => bit_functions::g,
            2 => bit_functions::h,
            3 => bit_functions::i,
            _ => unreachable!(),
        };

        let (mut k, inc) = consts::X_INDEX_START[round];
        let mut i: usize;
        let mut s: usize;

        for iter in 0..16 {
            i = 16 * round + iter;
            s = consts::S[round][iter % 4];

            *state.get_a() = (state
                .a()
                .wrapping_add(function(state.b(), state.c(), state.d()))
                .wrapping_add(block[k])
                .wrapping_add(consts::T[i]))
            .rotate_left(s as u32)
            .wrapping_add(state.b());

            state.next_state();
            k += inc;
            k %= 16;
        }
    }

    pub fn to_str(&self) -> String {
        format!("{:x}", self.0)
    }
}

impl std::ops::Deref for Md5 {
    type Target = u128;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_padding() {
        let mut vec = vec![0; 64];
        vec[0] = 0x80;
        assert_eq!(padding(&""), vec);

        vec[0] = 'a' as u8;
        vec[1] = 0x80;
        vec[64 - 8] = 0x8;
        assert_eq!(padding(&"a"), vec);
    }

    #[test]
    fn test_md5() {
        assert_eq!(*Md5::new(""), 0xd41d8cd98f00b204e9800998ecf8427e);
        assert_eq!(*Md5::new("a"), 0x0cc175b9c0f1b6a831c399e269772661);
        assert_eq!(*Md5::new("abc"), 0x900150983cd24fb0d6963f7d28e17f72);
        assert_eq!(
            *Md5::new("message digest"),
            0xf96b697d7cb7938d525a2f31aaf161d0
        );
        assert_eq!(
            *Md5::new("abcdefghijklmnopqrstuvwxyz"),
            0xc3fcd3d76192e4007dfb496cca67e13b
        );
        assert_eq!(
            *Md5::new("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789"),
            0xd174ab98d277d9f5a5611c2c9f419d9f
        );
        assert_eq!(
            *Md5::new(
                "12345678901234567890123456789012345678901234567890123456789012345678901234567890"
            ),
            0x57edf4a22be3c955ac49da2e2107b67a
        );
    }
}
