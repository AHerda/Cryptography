use std::sync::{
    Arc, Mutex,
    atomic::{AtomicBool, AtomicU64, Ordering},
};
use std::thread;
use std::time::Duration;

use rand::{self, Rng};

use crate::bit_functions::{self, *};
use crate::{
    consts::{self, Mask},
    md5::Md5,
    state::State,
};

pub struct CollisionFinder {
    m0: [u32; 16],
    m0_prim: [u32; 16],
}

impl CollisionFinder {
    pub fn new(m0: [u32; 16], m0_prim: [u32; 16]) -> Self {
        Self {
            m0,
            m0_prim,
        }
    }

    fn _random_message() -> [u32; 16] {
        let mut rng = rand::rng();
        let mut m1 = [0_u32; 16];
        for cell in &mut m1 {
            *cell = rng.random();
        }
        m1
    }

    #[inline]
    fn modify_bit(value_to_modify: u32, value_to_copy: u32, mask: &Mask) -> u32 {
        (value_to_modify & !mask.zero & !mask.copy & !mask.copy_not)
            | mask.one
            | (mask.copy & value_to_copy)
            | (mask.copy_not & !value_to_copy)
    }

    #[inline]
    fn q_17_to_1_21(q: &[u32; 65], masks: &[Mask]) -> bool {
        (17..=21).all(|i| Self::check_q(q[i], q[i - 1], &masks[i - 1]))
    }

    #[inline]
    fn check_q(q: u32, q_prev: u32, mask: &Mask) -> bool {
        q & mask.zero == 0
            && q & mask.one == mask.one
            && q & mask.copy == q_prev & mask.copy
            && q & mask.copy_not == !q_prev & mask.copy_not
    }

    fn process_message(state: &State, state2: &State, c: Arc<AtomicU64>) -> Option<[u32; 16]> {
        let s = &consts::S;
        let t = &consts::T;
        let x = &consts::X_INDEX_START;
        let masks = &consts::MASKS;
        let mut rng = rand::rng();
        let mut m1 = [0; 16];

        macro_rules! sixteen {
            ($func: ident, $a: expr, $b: expr, $c: expr, $d: expr, $k: expr, $s: expr, $i: expr, $orig: expr) => {
                $a = ($orig
                    .wrapping_add($func($b, $c, $d))
                    .wrapping_add(m1[$k])
                    .wrapping_add(t[$i]))
                .rotate_left($s as u32)
                .wrapping_add($b)
            };
        }

        macro_rules! inverse {
            ($func: ident, $a: expr, $b: expr, $c: expr, $d: expr, $k: expr, $s: expr, $i: expr, $orig: expr) => {
                m1[$k] = $a
                    .wrapping_sub($b)
                    .rotate_right($s as u32)
                    .wrapping_sub($orig)
                    .wrapping_sub($func($b, $c, $d))
                    .wrapping_sub(t[$i])
            };
        }

        macro_rules! increment {
            ($k: expr, $inc: expr, $i: expr) => {
                $i += 1;
                $k = ($k + $inc) % 16;
            };
        }

        'main: loop {
            let mut q = [0_u32; 65];
            q[0] = state.b;

            // 1. choose Q_2, ..., Q_16 fullfining conditions
            q[2] = Self::modify_bit(rng.random(), 0x0000_0000, &masks[1]);
            q[2] = (q[2] & !0x8000_0000) | (!state.b & 0x8000_0000);
            for i in 3..=16 {
                q[i] = Self::modify_bit(rng.random(), q[i - 1], &masks[i - 1]);
            }

            // 2. Calculate m_5, .. m_15
            for i in 5..=15 {
                inverse!(
                    f,
                    q[i + 1],
                    q[i],
                    q[i - 1],
                    q[i - 2],
                    i,
                    s[0][i % 4],
                    i,
                    q[i - 3]
                )
            }

            // 3. Loop until Q_17, ..., Q_21 are fullfilling conditions
            for iter in 0..(1 << 12) {
                // 3.a) Choose Q_1 fullfiling conditions
                q[1] = Self::modify_bit(rng.random(), q[0], &masks[0]);
                q[1] = (q[1] & !masks[1].copy) | (q[2] & masks[1].copy);
                if !Self::check_q(q[1], q[0], &masks[0]) {
                    continue;
                }

                // 3.b) Calculate m_0, ... m_4
                inverse!(f, q[1], state.b, state.c, state.d, 0, s[0][0], 0, state.a); // m_0
                inverse!(f, q[2], q[1], state.b, state.c, 1, s[0][1], 1, state.d); // m_1
                inverse!(f, q[3], q[2], q[1], state.b, 2, s[0][2], 2, state.c); // m_2
                inverse!(f, q[4], q[3], q[2], q[1], 3, s[0][3], 3, state.b); // m_3
                inverse!(f, q[5], q[4], q[3], q[2], 4, s[0][0], 4, q[1]); // m_4

                // 3.c) Calculate Q_17, ..., Q_21
                let mut i = 16;
                let (mut k, inc) = x[1];
                sixteen!(g, q[i + 1], q[i], q[i - 1], q[i - 2], k, s[1][i % 4], i, q[i - 3]);
                increment!(k, inc, i);
                sixteen!(g, q[i + 1], q[i], q[i - 1], q[i - 2], k, s[1][i % 4], i, q[i - 3]);
                increment!(k, inc, i);
                sixteen!(g, q[i + 1], q[i], q[i - 1], q[i - 2], k, s[1][i % 4], i, q[i - 3]);
                increment!(k, inc, i);
                sixteen!(g, q[i + 1], q[i], q[i - 1], q[i - 2], k, s[1][i % 4], i, q[i - 3]);
                increment!(k, inc, i);
                sixteen!(g, q[i + 1], q[i], q[i - 1], q[i - 2], k, s[1][i % 4], i, q[i - 3]);

                if Self::q_17_to_1_21(&q, masks) {
                    break;
                }

                if iter == (1 << 12) - 1 {
                    continue 'main;
                }
            }

            // Check for free bits in q9 and q10
            let q9_free_mask = !(masks[8].fixed_bits() | !q[11]);
            let q10_free_mask = !(masks[9].fixed_bits() | q[11]);

            let mut q9_free_count = q9_free_mask.count_ones() as usize;
            let mut q10_free_count = q10_free_mask.count_ones() as usize;

            let q9_max = 1 << q9_free_count;
            let q10_max = 1 << q10_free_count;

            let mut q9_free_indecies = vec![];
            let mut q10_free_indecies = vec![];

            for i in 0..32_u32 {
                let bit = 1 << i;
                if q9_free_mask & bit != 0 {q9_free_indecies.push(i)}
                if q10_free_mask & bit != 0 {q10_free_indecies.push(i)}
            }

            while q10_free_count + q10_free_count > 15 {
                if q9_free_count >= q10_free_count && q9_free_count > 0 {
                    q9_free_count -= 1;
                } else if q10_free_count > 0 {
                    q10_free_count -= 1;
                }
            }

            // 4. Loop over all possible Q_9, Q_10 satisfying conditions such that m_11 does not change
            let q9_base = q[9];
            let q10_base = q[10];

            for mask9 in 0..q9_max {
                // Calculating next q9 value that respects the fixed bits
                let mut q9_candidate = q9_base;
                for i in 0..q9_free_count {
                    let bit = 1 << q9_free_indecies[i];
                    if mask9 & (1 << i) != 0 {
                        q9_candidate |= bit;
                    } else {
                        q9_candidate &= !bit;
                    }
                }

                for mask10 in 0..q10_max {
                    // Calculating next q10 value that respects the fixed bits
                    let mut q10_candidate = q10_base;
                    for i in 0..q10_free_count {
                        let bit = 1 << q10_free_indecies[i];
                        if mask10 & (1 << i) != 0 {
                            q10_candidate |= bit;
                        } else {
                            q10_candidate &= !bit;
                        }
                    }

                    let m11 = q[12]
                        .wrapping_sub(q[11])
                        .rotate_right(s[0][11 % 4] as u32)
                        .wrapping_sub(q[8])
                        .wrapping_sub(f(q[11], q10_candidate, q9_candidate))
                        .wrapping_sub(t[11]);

                    if m11 != m1[11] { continue }

                    q[9] = q9_candidate;
                    q[10] = q10_candidate;

                    // 4.a) Calculate m_8, m_9, m_10, m_12, m_13 -> we skip m_11
                    for i in 8..=13 {
                        inverse!(f, q[i + 1], q[i], q[i - 1], q[i - 2], i, s[0][i % 4], i, q[i - 3]);
                    }

                    // 4.b) Calculate Q_22, ..., Q_64
                    // 4.c) Verify conditions on Q_22, ..., Q_64, T_22, T_34
                    let oldest_bit = 1 << 31;
                    let mut i = 21;
                    let (mut k, inc) = x[1];
                    k += 5 * inc;
                    k %= 16;

                    sixteen!(g, q[i + 1], q[i], q[i - 1], q[i - 2], k, s[1][i % 4], i, q[i - 3]); // Q_22
                    if !Self::check_q(q[i + 1], q[i], &masks[i]) {
                        continue;
                    }
                    let t22 = q[i + 1].wrapping_sub(q[i]).rotate_right(s[1][i % 4] as u32);
                    if t22 & (1 << 17) != 0 { continue }
                    increment!(k, inc, i);
                    sixteen!(g, q[i + 1], q[i], q[i - 1], q[i - 2], k, s[1][i % 4], i, q[i - 3]); // Q_23
                    if !Self::check_q(q[i + 1], q[i], &masks[i]) {
                        continue;
                    }
                    increment!(k, inc, i);
                    sixteen!(g, q[i + 1], q[i], q[i - 1], q[i - 2], k, s[1][i % 4], i, q[i - 3]); // Q_24
                    if !Self::check_q(q[i + 1], q[i], &masks[i]) {
                        continue;
                    }
                    increment!(k, inc, i);

                    sixteen!(g, q[i + 1], q[i], q[i - 1], q[i - 2], k, s[1][i % 4], i, q[i - 3]); // Q_25
                    increment!(k, inc, i);
                    sixteen!(g, q[i + 1], q[i], q[i - 1], q[i - 2], k, s[1][i % 4], i, q[i - 3]); // Q_26
                    increment!(k, inc, i);
                    sixteen!(g, q[i + 1], q[i], q[i - 1], q[i - 2], k, s[1][i % 4], i, q[i - 3]); // Q_27
                    increment!(k, inc, i);
                    sixteen!(g, q[i + 1], q[i], q[i - 1], q[i - 2], k, s[1][i % 4], i, q[i - 3]); // Q_28
                    increment!(k, inc, i);
                    sixteen!(g, q[i + 1], q[i], q[i - 1], q[i - 2], k, s[1][i % 4], i, q[i - 3]); // Q_29
                    increment!(k, inc, i);
                    sixteen!(g, q[i + 1], q[i], q[i - 1], q[i - 2], k, s[1][i % 4], i, q[i - 3]); // Q_30
                    increment!(k, inc, i);
                    sixteen!(g, q[i + 1], q[i], q[i - 1], q[i - 2], k, s[1][i % 4], i, q[i - 3]); // Q_31
                    increment!(k, inc, i);
                    sixteen!(g, q[i + 1], q[i], q[i - 1], q[i - 2], k, s[1][i % 4], i, q[i - 3]); // Q_32
                    i += 1;

                    // round 3
                    let (mut k, inc) = x[2];
                    sixteen!(h, q[i + 1], q[i], q[i - 1], q[i - 2], k, s[2][i % 4], i, q[i - 3]); // Q_33
                    increment!(k, inc, i);
                    sixteen!(h, q[i + 1], q[i], q[i - 1], q[i - 2], k, s[2][i % 4], i, q[i - 3]); // Q_34
                    let t34 = q[i + 1].wrapping_sub(q[i]).rotate_right(s[2][i % 4] as u32);
                    if t34 & (1 << 15) != 0 { continue }
                    increment!(k, inc, i);
                    sixteen!(h, q[i + 1], q[i], q[i - 1], q[i - 2], k, s[2][i % 4], i, q[i - 3]); // Q_35
                    increment!(k, inc, i);
                    sixteen!(h, q[i + 1], q[i], q[i - 1], q[i - 2], k, s[2][i % 4], i, q[i - 3]); // Q_36
                    increment!(k, inc, i);
                    sixteen!(h, q[i + 1], q[i], q[i - 1], q[i - 2], k, s[2][i % 4], i, q[i - 3]); // Q_37
                    increment!(k, inc, i);
                    sixteen!(h, q[i + 1], q[i], q[i - 1], q[i - 2], k, s[2][i % 4], i, q[i - 3]); // Q_38
                    increment!(k, inc, i);
                    sixteen!(h, q[i + 1], q[i], q[i - 1], q[i - 2], k, s[2][i % 4], i, q[i - 3]); // Q_39
                    increment!(k, inc, i);
                    sixteen!(h, q[i + 1], q[i], q[i - 1], q[i - 2], k, s[2][i % 4], i, q[i - 3]); // Q_40
                    increment!(k, inc, i);
                    sixteen!(h, q[i + 1], q[i], q[i - 1], q[i - 2], k, s[2][i % 4], i, q[i - 3]); // Q_41
                    increment!(k, inc, i);
                    sixteen!(h, q[i + 1], q[i], q[i - 1], q[i - 2], k, s[2][i % 4], i, q[i - 3]); // Q_42
                    increment!(k, inc, i);
                    sixteen!(h, q[i + 1], q[i], q[i - 1], q[i - 2], k, s[2][i % 4], i, q[i - 3]); // Q_43
                    increment!(k, inc, i);
                    sixteen!(h, q[i + 1], q[i], q[i - 1], q[i - 2], k, s[2][i % 4], i, q[i - 3]); // Q_44
                    increment!(k, inc, i);
                    sixteen!(h, q[i + 1], q[i], q[i - 1], q[i - 2], k, s[2][i % 4], i, q[i - 3]); // Q_45
                    increment!(k, inc, i);
                    sixteen!(h, q[i + 1], q[i], q[i - 1], q[i - 2], k, s[2][i % 4], i, q[i - 3]); // Q_46
                    increment!(k, inc, i);
                    sixteen!(h, q[i + 1], q[i], q[i - 1], q[i - 2], k, s[2][i % 4], i, q[i - 3]); // Q_47
                    increment!(k, inc, i);
                    sixteen!(h, q[i + 1], q[i], q[i - 1], q[i - 2], k, s[2][i % 4], i, q[i - 3]); // Q_48
                    if q[i + 1] & oldest_bit != q[i - 1] & oldest_bit { continue }
                    i += 1;

                    // round 4
                    let (mut k, inc) = x[3];
                    let ii = bit_functions::i;
                    sixteen!(ii, q[i + 1], q[i], q[i - 1], q[i - 2], k, s[3][i % 4], i, q[i - 3]); // Q_49
                    if q[i + 1] & oldest_bit != q[i - 1] & oldest_bit { continue }
                    increment!(k, inc, i);
                    sixteen!(ii, q[i + 1], q[i], q[i - 1], q[i - 2], k, s[3][i % 4], i, q[i - 3]); // Q_50
                    if q[i + 1] & oldest_bit == q[i - 1] & oldest_bit { continue }
                    increment!(k, inc, i);
                    sixteen!(ii, q[i + 1], q[i], q[i - 1], q[i - 2], k, s[3][i % 4], i, q[i - 3]); // Q_51
                    if q[i + 1] & oldest_bit != q[i - 1] & oldest_bit { continue }
                    increment!(k, inc, i);
                    sixteen!(ii, q[i + 1], q[i], q[i - 1], q[i - 2], k, s[3][i % 4], i, q[i - 3]); // Q_52
                    if q[i + 1] & oldest_bit != q[i - 1] & oldest_bit { continue }
                    increment!(k, inc, i);
                    sixteen!(ii, q[i + 1], q[i], q[i - 1], q[i - 2], k, s[3][i % 4], i, q[i - 3]); // Q_53
                    if q[i + 1] & oldest_bit != q[i - 1] & oldest_bit { continue }
                    increment!(k, inc, i);
                    sixteen!(ii, q[i + 1], q[i], q[i - 1], q[i - 2], k, s[3][i % 4], i, q[i - 3]); // Q_54
                    if q[i + 1] & oldest_bit != q[i - 1] & oldest_bit { continue }
                    increment!(k, inc, i);
                    sixteen!(ii, q[i + 1], q[i], q[i - 1], q[i - 2], k, s[3][i % 4], i, q[i - 3]); // Q_55
                    if q[i + 1] & oldest_bit != q[i - 1] & oldest_bit { continue }
                    increment!(k, inc, i);
                    sixteen!(ii, q[i + 1], q[i], q[i - 1], q[i - 2], k, s[3][i % 4], i, q[i - 3]); // Q_56
                    if q[i + 1] & oldest_bit != q[i - 1] & oldest_bit { continue }
                    increment!(k, inc, i);
                    sixteen!(ii, q[i + 1], q[i], q[i - 1], q[i - 2], k, s[3][i % 4], i, q[i - 3]); // Q_57
                    if q[i + 1] & oldest_bit != q[i - 1] & oldest_bit { continue }
                    increment!(k, inc, i);
                    sixteen!(ii, q[i + 1], q[i], q[i - 1], q[i - 2], k, s[3][i % 4], i, q[i - 3]); // Q_58
                    if q[i + 1] & oldest_bit != q[i - 1] & oldest_bit { continue }
                    increment!(k, inc, i);
                    sixteen!(ii, q[i + 1], q[i], q[i - 1], q[i - 2], k, s[3][i % 4], i, q[i - 3]); // Q_59
                    if q[i + 1] & oldest_bit != q[i - 1] & oldest_bit { continue }
                    increment!(k, inc, i);
                    sixteen!(ii, q[i + 1], q[i], q[i - 1], q[i - 2], k, s[3][i % 4], i, q[i - 3]); // Q_60
                    if q[i + 1] & oldest_bit == q[i - 1] & oldest_bit { continue }
                    increment!(k, inc, i);
                    sixteen!(ii, q[i + 1], q[i], q[i - 1], q[i - 2], k, s[3][i % 4], i, q[i - 3]); // Q_61
                    if q[i + 1] & oldest_bit != q[i - 1] & oldest_bit { continue }
                    increment!(k, inc, i);
                    sixteen!(ii, q[i + 1], q[i], q[i - 1], q[i - 2], k, s[3][i % 4], i, q[i - 3]); // Q_62
                    if q[i + 1] & oldest_bit != q[i - 1] & oldest_bit { continue }
                    increment!(k, inc, i);
                    sixteen!(ii, q[i + 1], q[i], q[i - 1], q[i - 2], k, s[3][i % 4], i, q[i - 3]); // Q_63
                    if q[i + 1] & oldest_bit != q[i - 1] & oldest_bit { continue }
                    increment!(k, inc, i);
                    sixteen!(ii, q[i + 1], q[i], q[i - 1], q[i - 2], k, s[3][i % 4], i, q[i - 3]); // Q_64

                    let m1p: Vec<u32> = m1
                        .iter()
                        .zip(consts::DIFF_M1)
                        .map(|(&x, y)| ((x as i64 + y) % (1 << 32)) as u32).collect();

                    let h = Md5::new_with_state_raw_block(&m1, state.clone());
                    let hp = Md5::new_with_state_raw_block(&m1p, state2.clone());

                    if h == hp {
                        return Some(m1);
                    } else {
                        c.fetch_add(1, Ordering::Relaxed);
                        continue 'main;
                    }
                }

                if mask9 == q9_max - 1 {
                    continue 'main;
                }
            }
        }
    }

    fn _log_data(counter: Arc<AtomicU64>, counter_near: Arc<AtomicU64>, found: Arc<AtomicBool>) {
        thread::spawn(move || {
            while !found.load(Ordering::Relaxed) {
                thread::sleep(Duration::from_secs(5));
                let c = counter.load(Ordering::Relaxed);
                let c2 = counter_near.load(Ordering::Relaxed);
                println!(
                    "{} -> Progress: {} / {}, ({:.6}%), Found near collisions: {}",
                    chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                    c,
                    1_u64 << 37,
                    (c as f64 / (1_u64 << 37) as f64) * 100.0,
                    c2,
                );
            }
        });
    }

    pub fn find_collision(&self) -> Vec<[u32; 16]> {
        let iv_0 = Md5::new_raw_block(&self.m0).get_state();
        let iv_0_prim = Md5::new_raw_block(&self.m0_prim).get_state();

        let result: Arc<Mutex<Vec<[u32; 16]>>> = Arc::new(Mutex::new(vec![]));
        let counter_near: Arc<AtomicU64> = Arc::new(AtomicU64::new(0));

        thread::scope(|s| {
            for _ in 0..12 {s.spawn(|| {
                if let Some(m1) = Self::process_message(&iv_0, &iv_0_prim, counter_near.clone()) {
                    result.lock().unwrap().push(m1);
                }
            });}
        });
        let result = result.lock().unwrap().to_vec();
        println!("Found collisions: {}", result.len());
        println!("Found near collisions: {}", counter_near.load(Ordering::Relaxed));
        result
    }
}
