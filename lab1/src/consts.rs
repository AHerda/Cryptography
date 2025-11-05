pub const STARTING_A: u32 = 0x67452301;
pub const STARTING_B: u32 = 0xEFCDAB89;
pub const STARTING_C: u32 = 0x98BADCFE;
pub const STARTING_D: u32 = 0x10325476;

pub const T: [u32; 64] = [
    0xd76aa478, 0xe8c7b756, 0x242070db, 0xc1bdceee, 0xf57c0faf, 0x4787c62a, 0xa8304613, 0xfd469501,
    0x698098d8, 0x8b44f7af, 0xffff5bb1, 0x895cd7be, 0x6b901122, 0xfd987193, 0xa679438e, 0x49b40821,
    0xf61e2562, 0xc040b340, 0x265e5a51, 0xe9b6c7aa, 0xd62f105d, 0x2441453, 0xd8a1e681, 0xe7d3fbc8,
    0x21e1cde6, 0xc33707d6, 0xf4d50d87, 0x455a14ed, 0xa9e3e905, 0xfcefa3f8, 0x676f02d9, 0x8d2a4c8a,
    0xfffa3942, 0x8771f681, 0x6d9d6122, 0xfde5380c, 0xa4beea44, 0x4bdecfa9, 0xf6bb4b60, 0xbebfbc70,
    0x289b7ec6, 0xeaa127fa, 0xd4ef3085, 0x4881d05, 0xd9d4d039, 0xe6db99e5, 0x1fa27cf8, 0xc4ac5665,
    0xf4292244, 0x432aff97, 0xab9423a7, 0xfc93a039, 0x655b59c3, 0x8f0ccc92, 0xffeff47d, 0x85845dd1,
    0x6fa87e4f, 0xfe2ce6e0, 0xa3014314, 0x4e0811a1, 0xf7537e82, 0xbd3af235, 0x2ad7d2bb, 0xeb86d391,
];

pub const S: [[usize; 4]; 4] = [
    [7, 12, 17, 22],
    [5, 9, 14, 20],
    [4, 11, 16, 23],
    [6, 10, 15, 21],
];

/// While processing 16-word block this are the indecies this will help
/// calculate indecies of the word we should be focusing on.<br>
/// (starting_index, increment_modulo_16)
pub const X_INDEX_START: [(usize, usize); 4] = [(0, 1), (1, 5), (5, 3), (0, 7)];

/// This are the initial values M_0 and M'_0 and M_1 and M'_1 from table2 in source from excercise
/// Version 1
pub const M0_1: [u32; 16] = [
    0x2dd31d1, 0xc4eee6c5, 0x69a3d69, 0x5cf9af98, 0x87b5ca2f, 0xab7e4612, 0x3e580440, 0x897ffbb8,
    0x634ad55, 0x2b3f409, 0x8388e483, 0x5a417125, 0xe8255108, 0x9fc9cdf7, 0xf2bd1dd9, 0x5b3c3780,
];
pub const M0_PRIM_1: [u32; 16] = [
    0x2dd31d1, 0xc4eee6c5, 0x69a3d69, 0x5cf9af98, 0x7b5ca2f, 0xab7e4612, 0x3e580440, 0x897ffbb8,
    0x634ad55, 0x2b3f409, 0x8388e483, 0x5a41f125, 0xe8255108, 0x9fc9cdf7, 0x72bd1dd9, 0x5b3c3780,
];
pub const M1_1: [u32; 16] = [
    0xd11d0b96, 0x9c7b41dc, 0xf497d8e4, 0xd555655a, 0xc79a7335, 0xcfdebf0, 0x66f12930, 0x8fb109d1,
    0x797f2775, 0xeb5cd530, 0xbaade822, 0x5c15cc79, 0xddcb74ed, 0x6dd3c55f, 0xd80a9bb1, 0xe3a7cc35,
];
pub const M1_PRIM_1: [u32; 16] = [
    0xd11d0b96, 0x9c7b41dc, 0xf497d8e4, 0xd555655a, 0x479a7335, 0xcfdebf0, 0x66f12930, 0x8fb109d1,
    0x797f2775, 0xeb5cd530, 0xbaade822, 0x5c154c79, 0xddcb74ed, 0x6dd3c55f, 0x580a9bb1, 0xe3a7cc35,
];
pub const EXPECTED_HASH1: &str = "9603161fa30f9dbf9f65ffbcf41fc7ef";

/// Version2
pub const M0_2: [u32; 16] = [
    0x2dd31d1, 0xc4eee6c5, 0x69a3d69, 0x5cf9af98, 0x87b5ca2f, 0xab7e4612, 0x3e580440, 0x897ffbb8,
    0x634ad55, 0x2b3f409, 0x8388e483, 0x5a417125, 0xe8255108, 0x9fc9cdf7, 0xf2bd1dd9, 0x5b3c3780,
];
pub const M0_PRIM_2: [u32; 16] = [
    0x2dd31d1, 0xc4eee6c5, 0x69a3d69, 0x5cf9af98, 0x7b5ca2f, 0xab7e4612, 0x3e580440, 0x897ffbb8,
    0x634ad55, 0x2b3f409, 0x8388e483, 0x5a41f125, 0xe8255108, 0x9fc9cdf7, 0x72bd1dd9, 0x5b3c3780,
];
pub const M1_2: [u32; 16] = [
    0x313e82d8, 0x5b8f3456, 0xd4ac6dae, 0xc619c936, 0xb4e253dd, 0xfd03da87, 0x6633902, 0xa0cd48d2,
    0x42339fe9, 0xe87e570f, 0x70b654ce, 0x1e0da880, 0xbc2198c6, 0x9383a8b6, 0x2b65f996, 0x702af76f,
];
pub const M1_PRIM_2: [u32; 16] = [
    0x313e82d8, 0x5b8f3456, 0xd4ac6dae, 0xc619c936, 0x34e253dd, 0xfd03da87, 0x6633902, 0xa0cd48d2,
    0x42339fe9, 0xe87e570f, 0x70b654ce, 0x1e0d2880, 0xbc2198c6, 0x9383a8b6, 0xab65f996, 0x702af76f,
];
pub const EXPECTED_HASH2: &str = "8d5e701961804e08715d6b586324c015";

/// Diffs
pub const DIFF_M0: [i64; 16] = [
    0,
    0,
    0,
    0,
    1 << 31,
    0,
    0,
    0,
    0,
    0,
    0,
    1 << 15,
    0,
    0,
    1 << 31,
    0,
];
pub const DIFF_M1: [i64; 16] = [
    0,
    0,
    0,
    0,
    1 << 31,
    0,
    0,
    0,
    0,
    0,
    0,
    -(1 << 15),
    0,
    0,
    1 << 31,
    0,
];

pub struct Mask {
    pub one: u32,
    pub zero: u32,
    pub copy_prev: Option<u32>,
}

/// Masks
pub const MASKS: [Mask; 16] = [
    Mask {
        one: 0x84200000,
        zero: 0x0A000820,
        copy_prev: None,
    },
    Mask {
        one: 0x8C000800,
        zero: 0x02208026,
        copy_prev: Some(0x701F10C0),
    },
    Mask {
        one: 0xBE1F0966,
        zero: 0x40201080,
        copy_prev: Some(0x00000018),
    },
    Mask {
        one: 0xBA040010,
        zero: 0x443B19EE,
        copy_prev: Some(0x00000601),
    },
    Mask {
        one: 0x482F0E50,
        zero: 0xB41011AF,
        copy_prev: None,
    },
    Mask {
        one: 0x04220C56,
        zero: 0x9A1113A9,
        copy_prev: None,
    },
    Mask {
        one: 0x96011E01,
        zero: 0x083201C0,
        copy_prev: Some(0x01808000),
    },
    Mask {
        one: 0x843283C0,
        zero: 0x1B810001,
        copy_prev: Some(0x00000002),
    },
    Mask {
        one: 0x9C0101C1,
        zero: 0x03828202,
        copy_prev: Some(0x00001000),
    },
    Mask {
        one: 0x878383C0,
        zero: 0x00041003,
        copy_prev: None,
    },
    Mask {
        one: 0x800583C3,
        zero: 0x00021000,
        copy_prev: Some(0x00086000),
    },
    Mask {
        one: 0x80081080,
        zero: 0x0007E000,
        copy_prev: Some(0x7F000000),
    },
    Mask {
        one: 0x3F0FE008,
        zero: 0xC0000080,
        copy_prev: None,
    },
    Mask {
        one: 0x400BE088,
        zero: 0xBF040000,
        copy_prev: None,
    },
    Mask {
        one: 0x7D000000,
        zero: 0x82008008,
        copy_prev: None,
    },
    Mask {
        one: 0x20000000,
        zero: 0x80000000,
        copy_prev: None,
    },
];
