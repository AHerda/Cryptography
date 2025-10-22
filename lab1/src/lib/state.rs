use super::consts;

#[derive(Debug, Clone, Copy)]
pub struct State {
    pub a: u32,
    pub b: u32,
    pub c: u32,
    pub d: u32,
    state: u8,
}

impl State {
    pub fn new() -> Self {
        Self {
            a: consts::STARTING_A,
            b: consts::STARTING_B,
            c: consts::STARTING_C,
            d: consts::STARTING_D,
            state: 0,
        }
    }

    pub fn a(&self) -> u32 {
        match self.state {
            0 => self.a,
            1 => self.d,
            2 => self.c,
            3 => self.b,
            _ => unreachable!(),
        }
    }
    pub fn b(&self) -> u32 {
        match self.state {
            0 => self.b,
            1 => self.a,
            2 => self.d,
            3 => self.c,
            _ => unreachable!(),
        }
    }
    pub fn c(&self) -> u32 {
        match self.state {
            0 => self.c,
            1 => self.b,
            2 => self.a,
            3 => self.d,
            _ => unreachable!(),
        }
    }
    pub fn d(&self) -> u32 {
        match self.state {
            0 => self.d,
            1 => self.c,
            2 => self.b,
            3 => self.a,
            _ => unreachable!(),
        }
    }
    pub fn get_a(&mut self) -> &mut u32 {
        match self.state {
            0 => &mut self.a,
            1 => &mut self.d,
            2 => &mut self.c,
            3 => &mut self.b,
            _ => unreachable!(),
        }
    }

    pub fn next_state(&mut self) {
        self.state = (self.state + 1) % 4;
    }

    pub fn get_hash(&self) -> u128 {
        (( self.a.to_le() as u128 ) << 48) | (( self.b.to_le() as u128 ) << 32) | (( self.c.to_le() as u128 ) << 16) | (self.d.to_le() as u128)
    }
}

impl std::ops::AddAssign<State> for State {
    fn add_assign(&mut self, rhs: State) {
        self.a += rhs.a;
        self.b += rhs.b;
        self.c += rhs.c;
        self.d += rhs.d;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a() {
        let mut state = State::new();

        assert_eq!(state.a(), consts::STARTING_A);
        state.next_state();
        assert_eq!(state.a(), consts::STARTING_D);
        state.next_state();
        assert_eq!(state.a(), consts::STARTING_C);
        state.next_state();
        assert_eq!(state.a(), consts::STARTING_B);
        state.next_state();
        assert_eq!(state.a(), consts::STARTING_A);
    }

    #[test]
    fn test_b() {
        let mut state = State::new();

        assert_eq!(state.b(), consts::STARTING_B);
        state.next_state();
        assert_eq!(state.b(), consts::STARTING_A);
        state.next_state();
        assert_eq!(state.b(), consts::STARTING_D);
        state.next_state();
        assert_eq!(state.b(), consts::STARTING_C);
        state.next_state();
        assert_eq!(state.b(), consts::STARTING_B);
    }

    #[test]
    fn test_c() {
        let mut state = State::new();

        assert_eq!(state.c(), consts::STARTING_C);
        state.next_state();
        assert_eq!(state.c(), consts::STARTING_B);
        state.next_state();
        assert_eq!(state.c(), consts::STARTING_A);
        state.next_state();
        assert_eq!(state.c(), consts::STARTING_D);
        state.next_state();
        assert_eq!(state.c(), consts::STARTING_C);
    }

    #[test]
    fn test_d() {
        let mut state = State::new();

        assert_eq!(state.d(), consts::STARTING_D);
        state.next_state();
        assert_eq!(state.d(), consts::STARTING_C);
        state.next_state();
        assert_eq!(state.d(), consts::STARTING_B);
        state.next_state();
        assert_eq!(state.d(), consts::STARTING_A);
        state.next_state();
        assert_eq!(state.d(), consts::STARTING_D);
    }

    #[test]
    fn test_next_state() {
        let mut state = State::new();
        for _ in 0..40 {
            state.next_state();
        }
        assert_eq!(state.a(), consts::STARTING_A);
        assert_eq!(state.b(), consts::STARTING_B);
        assert_eq!(state.c(), consts::STARTING_C);
        assert_eq!(state.d(), consts::STARTING_D);
    }
}
