use rayon::prelude::*;
use std::sync::{
    Arc, Mutex,
    atomic::{AtomicBool, AtomicU64, Ordering},
};
use std::thread;
use std::time::Duration;

use rand::{self, Rng, rng};

use crate::bit_functions::*;
use crate::{
    consts::{self, Mask},
    md5::Md5,
    state::State,
};

pub struct CollisionFinder {
    m0: [u32; 16],
    m0_prim: [u32; 16],
    delta_m1: [i64; 16],
}

impl CollisionFinder {
    pub fn new(m0: [u32; 16], m0_prim: [u32; 16], delta_m1: [i64; 16]) -> Self {
        Self {
            m0,
            m0_prim,
            delta_m1,
        }
    }

    fn random_message() -> [u32; 16] {
        let mut rng = rand::rng();
        let mut m1 = [0_u32; 16];
        for cell in &mut m1 {
            *cell = rng.random();
        }
        m1
    }

    fn modify_bit(value_to_modify: &mut u32, value_to_copy: u32, mask: &Mask) {
        *value_to_modify |= mask.one;
        *value_to_modify &= !mask.zero;
        *value_to_modify = (*value_to_modify & !mask.copy) | (value_to_copy & mask.copy);
        *value_to_modify = (*value_to_modify & !mask.copy_not) | (!value_to_copy & mask.copy_not);
    }

    fn q_17_to_1_21(q: &[u32; 65], masks: &[Mask]) -> bool {
        (17..=21).all(|i| {
            let Mask {zero, one, copy, copy_not} = masks[i - 1];
            q[i] & zero == 0
                && q[i] & one == one
                && q[i] & copy == q[i - 1] & copy
                && q[i] & copy_not == !q[i - 1] & copy_not
        })
    }

    fn process_message(m1: &mut [u32; 16], state: &State) -> Option<State> {
        let s = &consts::S;
        let t = &consts::T;
        let x = &consts::X_INDEX_START;
        let masks = &consts::MASKS;

        macro_rules! sixteen {
            ($func: ident, $a: expr, $b: ident, $c: ident, $d: ident, $k: expr, $s: expr, $i: expr, $orig: ident) => {
                $a = ($orig
                    .wrapping_add($func($b, $c, $d))
                    .wrapping_add(m1[$k])
                    .wrapping_add(t[$i]))
                .rotate_left($s as u32)
                .wrapping_add($b);
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

        let mut rng = rand::rng();
        let mut q = [0_u32; 65];
        q[0] = state.b;

        // 1. choose Q_2, ..., Q_16 fullfining conditions
        for i in 2..=16 {
            q[i] = rng.random();
            let value_to_copy = q[i - 1];
            Self::modify_bit(&mut q[i], value_to_copy, &masks[i - 1]);
        }

        // 2. Calculate m_5, .. m_15
        for i in 5..=15 {
            inverse!(f, q[i + 1], q[i], q[i - 1], q[i - 2], i, s[0][i % 4], i, q[i - 3])
        }

        let mut counter = 0;
        // 3. Loop until Q_17, ..., Q_21 are fullfilling conditions
        while !Self::q_17_to_1_21(&q, masks) {
            counter += 1;
            // println!("In Part 3");
            // 3.a) Choose Q_1 fullfiling conditions
            q[1] = rng.random();
            let value_to_copy = q[0];
            Self::modify_bit(&mut q[1], value_to_copy, &masks[0]);

            // 3.b) Calculate m_0, ... m_4
            inverse!(f, q[1], state.b, state.c, state.d, 0, s[0][0], 0, state.a); // m_0
            inverse!(f, q[2], q[1], state.b, state.c, 1, s[0][1], 1, state.d); // m_1
            inverse!(f, q[3], q[2], q[1], state.b, 2, s[0][2], 2, state.c); // m_2
            inverse!(f, q[4], q[3], q[2], q[1], 3, s[0][3], 3, state.b); // m_3
            inverse!(f, q[5], q[4], q[3], q[2], 4, s[0][0], 4, q[1]); // m_4

            // 3.c) Calculate Q_17, ..., Q_21
            let (mut k, inc) = x[1];
            let mut _dummy = 0;
            for i in 17..=21 {
                let (b, c, d, a_old) = (q[i - 1], q[i - 2], q[i - 3], q[i - 4]);
                sixteen!(g, q[i], b, c, d, k, s[1][(i - 1) % 4], i - 1, a_old);
                increment!(k, inc, _dummy);
            }
            if counter == 1 << 12 {
                return None;
            }
        }
        // println!("After part 3");

        // 4. Loop over all possible Q_9, Q_10 satisfying conditions such that m_11 does not change
        let f11 = q[12]
            .wrapping_sub(q[11])
            .rotate_right(s[0][11 % 4] as u32)
            .wrapping_sub(q[8])
            .wrapping_sub(m1[11])
            .wrapping_sub(t[11]);
        let mask = 0x80000000;
        let is = consts::I_IDNEXES;
        let js = consts::J_IDNEXES;
        let ks = consts::K_IDNEXES;

        let q9mask: Vec<u32> = (0..(1 << 5))
            .map(|k| {
                let msk = (k << 5) ^ (k << 13) ^ (k << 17) ^ (k << 24);
                (msk & 0x0008_4000) as u32
            })
            .collect();

        let q10mask: Vec<u32> = (0..(1 << 5))
            .map(|k| {
                let msk = (k << 5) ^ (k << 13) ^ (k << 17) ^ (k << 24);
                (msk & 0x1800_0020) as u32
            })
            .collect();

        q[9] = rng.random();
        q[9] = (q[9] & q[11]) | (f11 & q[11]);
        let temp = q[8];
        Self::modify_bit(&mut q[9], temp, &masks[8]);

        q[10] = rng.random();
        q[10] = (q[10] & !q[11]) | (f11 & !q[11]);
        let temp = q[9];
        Self::modify_bit(&mut q[10], temp, &masks[9]);

        let q9_base = q[9];
        let q10_base = q[10];

        'loop_4: for &m10_mask in &q10mask {
            q[10] = q10_base ^ m10_mask;

            for &m9_mask in &q9mask {
                q[9] = q9_base ^ m9_mask;

                // teraz (tak jak w oryginale) stosujemy modyfikację bitów zależną od poprzedniego Q:
                // (jeżeli chcesz nadal używać modify_bit: zastosuj ją _po_ ustawieniu kandydatów)
                let temp8 = q[8];
                Self::modify_bit(&mut q[9], temp8, &masks[8]);
                let temp9 = q[9];
                Self::modify_bit(&mut q[10], temp9, &masks[9]);

                // 4.a) Calculate m_8, m_9, m_10, m_12, m_13 -> we skip m_11
                inverse!(f, q[9], q[8], q[7], q[6], 8, s[0][8 % 4], 8, q[5]); // m_8
                inverse!(f, q[10], q[9], q[8], q[7], 9, s[0][9 % 4], 9, q[6]); // m_9
                inverse!(f, q[11], q[10], q[9], q[8], 10, s[0][10 % 4], 10, q[7]); // m_10
                inverse!(f, q[13], q[12], q[11], q[10], 12, s[0][12 % 4], 12, q[9]); // m_12
                inverse!(f, q[14], q[13], q[12], q[11], 13, s[0][13 % 4], 13, q[10]); // m_13

                // Calculate Q_22, ..., Q_64
                let mut round = 1;
                let functions = [f, g, h, i];
                let mut func = functions[round];
                let (mut k, mut inc) = x[round];
                k += inc * (22 - 16);
                k %= 16;

                let mut _dummy = 0;
                for i in 22..=64 {
                    let (b, c, d, a_old) = (q[i - 1], q[i - 2], q[i - 3], q[i - 4]);
                    sixteen!(func, q[i], b, c, d, k, s[round][(i - 1) % 4], i - 1, a_old);
                    increment!(k, inc, _dummy);

                    if i % 16 == 0 && i != 64 {
                        round += 1;
                        func = functions[round];
                        (k, inc) = x[round];
                    }
                }

                // Verify conditions on Q_22, ..., Q_64, T_22, T_34
                for i in 22..=24 {
                    let Mask {zero, one, copy, copy_not} = masks[i - 1];
                    if q[i] & zero != 0
                        || q[i] & one != one
                        || q[i] & copy != q[i - 1] & copy
                        || q[i] & copy_not != !q[i - 1] & copy_not
                    {
                        continue 'loop_4;
                    }
                }

                let i_bit = q[is[0]] & mask;
                let j_bit = q[js[0]] & mask;
                let k_bit = q[ks[0]] & mask;
                if k_bit == i_bit {
                    continue 'loop_4;
                }
                for i in is {
                    if q[i] & mask != i_bit {
                        continue 'loop_4;
                    }
                }
                for j in js {
                    if q[j] & mask != j_bit {
                        continue 'loop_4;
                    }
                }
                for k in ks {
                    if q[k] & mask != k_bit {
                        continue 'loop_4;
                    }
                }

                let (k, inc) = x[1];
                let t22_mask = 1 << 17;
                let t22 = g(q[22], q[21], q[20]) + q[19] + t[22] + m1[(k + inc * 22) % 16];
                let (k, inc) = x[2];
                let t34_mask = 1 << 15;
                let t34 = h(q[34], q[33], q[32]) + q[31] + t[34] + m1[(k + inc * 34) % 16];
                if t22 & t22_mask != 0 || t34 & t34_mask != 0 {
                    continue 'loop_4;
                }

                // Stop searching if all conditions are satisfied and a near-collision is verified
                return  Some(State::new_with_values(q[61], q[64], q[63], q[62]));
            }
        }

        None
    }

    fn log_data(counter: Arc<AtomicU64>, counter_near: Arc<AtomicU64>, found: Arc<AtomicBool>) {
        thread::spawn(move || {
            while !found.load(Ordering::Relaxed) {
                thread::sleep(Duration::from_secs(1800));
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

    pub fn find_collision(&self) -> [u32; 16] {
        let iv_0 = Md5::new_raw_block(&self.m0).get_state();
        let iv_0_prim = Md5::new_raw_block(&self.m0_prim).get_state();

        let found = Arc::new(AtomicBool::new(false));
        let result = Arc::new(Mutex::new([0u32; 16]));
        let counter = Arc::new(AtomicU64::new(0));
        let counter_near = Arc::new(AtomicU64::new(0));


        Self::log_data(counter.clone(), counter_near.clone(), found.clone());

        (0..(1_u64 << 40)).into_par_iter().for_each(|_| {
            counter.fetch_add(1, Ordering::Relaxed);
            let mut m1 = Self::random_message();
            let mut h = match Self::process_message(&mut m1, &iv_0) {
                Some(v) => v,
                None => return,
            };
            h += iv_0;

            let m1_prim: Vec<u32> = m1
                .iter()
                .zip(self.delta_m1.iter())
                .map(|(&x, &y)| {
                    ((x as i64 + y + (1 << 32)) % (1 << 32)) as u32
                })
                .collect();
            let h_prim = Md5::new_with_state_raw_block(&m1_prim, iv_0_prim);

            if h.get_hash() == h_prim.get_hash() {
                if Md5::new_with_state_raw_block(&m1, iv_0).get_hash()
                    == Md5::new_with_state_raw_block(&m1_prim, iv_0_prim).get_hash()
                {
                    println!(
                        "{} -> Gloria! Gloria! Hallelujah!!!\n\tFound!!! {:?}",
                        chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                        m1,
                    );
                    *result.lock().unwrap() = m1;
                    found.store(true, Ordering::Relaxed);
                    return;
                } else {
                    println!(
                        "{} -> False positive {:?}",
                        chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                        m1,
                    );
                }
            }
            counter_near.fetch_add(1, Ordering::Relaxed);
        });

        *result.lock().unwrap()
    }
}
