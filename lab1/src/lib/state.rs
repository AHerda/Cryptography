const STARTING_A: u32 = 0x67452301;
const STARTING_B: u32 = 0xEFCDAB89;
const STARTING_C: u32 = 0x98BADCFE;
const STARTING_D: u32 = 0x10325476;

struct State {
    a: u32,
    b: u32,
    c: u32,
    d: u32,
}

impl State {
    pub fn new() -> Self {
        State {
            a: STARTING_A,
            b: STARTING_B,
            c: STARTING_C,
            d: STARTING_D,
        }
    }
}