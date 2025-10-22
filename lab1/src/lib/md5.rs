use super::{
    state::State,
    bitFunctions,
    consts
};

fn round(state: &mut State, block: &[u8], round: usize) {
    let function = match round {
        0 => bitFunctions::f,
        1 => bitFunctions::g,
        2 => bitFunctions::h,
        3 => bitFunctions::i,
        _ => unreachable!(),
    };

    let (mut k, inc) = consts::X_INDEX_START[round];
    let mut i: usize;
    let mut s: usize;
    for iter in 0..16 {
        i = 16 * round + iter;
        s = consts::S[round][iter % 4];

        *state.get_a() = state.b() + ((state.a() + function(state.b(), state.c(), state.d() + block[k] as u32 + consts::T[i])) << s);

        state.next_state();
        k += inc;
        k %= 16;
    }
}


pub fn md5(input: &str) -> u128 {
    let mut input = input.as_bytes().to_vec();
    let len = input.len() as u64;
    let mut padding_len = 512 - (len as usize % 512) - 64;
    if padding_len == 0 { padding_len = 512 }
    let mut padding = vec![0_u8; padding_len];
    padding[0] = 0b1000_0000;
    padding.extend_from_slice(&(len & 0xffff).to_le_bytes());
    padding.extend_from_slice(&(len & (0xffff << 32)).to_le_bytes());

    input.append(&mut padding);

    let n = input.len();
    let mut state = State::new();

    for i in 0..(n / 16 - 1) {
        let block = &input[i * 16..(i + 1) * 16];
        let mut temp_state = state.clone();

        round(&mut temp_state, block, 0);
        round(&mut temp_state, block, 1);
        round(&mut temp_state, block, 2);
        round(&mut temp_state, block, 3);

        state += temp_state;
    }

    state.get_hash()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
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
