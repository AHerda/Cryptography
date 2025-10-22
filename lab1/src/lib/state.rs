use crate::lib::consts;

struct State {
    a: u32,
    b: u32,
    c: u32,
    d: u32,
}

impl State {
    pub fn new() -> Self {
        State {
            a: consts::STARTING_A,
            b: consts::STARTING_B,
            c: consts::STARTING_C,
            d: consts::STARTING_D,
        }
    }
}
