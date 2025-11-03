use super::consts;

#[derive(Debug, Clone, Copy)]
pub struct State {
    pub a: u32,
    pub b: u32,
    pub c: u32,
    pub d: u32,
}

impl State {
    pub fn new() -> Self {
        Self {
            a: consts::STARTING_A,
            b: consts::STARTING_B,
            c: consts::STARTING_C,
            d: consts::STARTING_D,
        }
    }

    pub fn get_hash(&self) -> u128 {
        (self.a.swap_bytes() as u128) << 96
            | (self.b.swap_bytes() as u128) << 64
            | (self.c.swap_bytes() as u128) << 32
            | (self.d.swap_bytes() as u128)
    }

    pub fn get_hash_be(&self) -> u128 {
        (self.a as u128) << 96
            | (self.b as u128) << 64
            | (self.c as u128) << 32
            | (self.d as u128)
    }
}

impl std::ops::AddAssign<State> for State {
    fn add_assign(&mut self, rhs: State) {
        self.a = self.a.wrapping_add(rhs.a);
        self.b = self.b.wrapping_add(rhs.b);
        self.c = self.c.wrapping_add(rhs.c);
        self.d = self.d.wrapping_add(rhs.d);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_hash() {
        let state = State {
            a: 0x01234567,
            b: 0x89abcdef,
            c: 0xfedcba98,
            d: 0x76543210,
        };
        assert_eq!(
            format!("{:x}", state.get_hash()),
            format!(
                "{:x}{:x}{:x}{:x}",
                state.a.swap_bytes(),
                state.b.swap_bytes(),
                state.c.swap_bytes(),
                state.d.swap_bytes()
            )
        );
    }
}
