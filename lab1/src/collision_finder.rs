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
        let mut rng = rng();
        let mut m1 = [0_u32; 16];
        for cell in &mut m1 {
            *cell = rng.random();
        }
        m1
    }

    fn modify_bit(value_to_modify: &mut u32, value_to_copy: u32, mask: &Mask) {
        *value_to_modify |= mask.one;
        *value_to_modify &= !mask.zero;
        if let Some(copy_mask) = mask.copy_prev {
            *value_to_modify = (*value_to_modify & !copy_mask) | (value_to_copy & copy_mask)
        }
    }

    fn process_message(m1: &mut [u32; 16], mut state: State) -> State {
        let s = &consts::S;
        let t = &consts::T;
        let x = &consts::X_INDEX_START;
        let masks = &consts::MASKS;

        macro_rules! sixteen {
            ($func: ident, $a: ident, $b: ident, $c: ident, $d: ident, $k: expr, $s: expr, $i: expr) => {
                state.$a = (state
                    .$a
                    .wrapping_add($func(state.$b, state.$c, state.$d))
                    .wrapping_add(m1[$k])
                    .wrapping_add(t[$i]))
                .rotate_left($s as u32)
                .wrapping_add(state.$b);
            };
        }

        macro_rules! backtrack {
            ($func: ident, $a: ident, $b: ident, $c: ident, $d: ident, $k: expr, $s: expr, $i: expr) => {
                Self::modify_bit(&mut state.a, state.b, &masks[$i]);
                m1[$k] = state
                    .$a
                    .wrapping_sub(state.$b)
                    .rotate_right($s as u32)
                    .wrapping_sub(state.$a)
                    .wrapping_sub($func(state.$b, state.$c, state.$d));
            };
        }

        macro_rules! increment {
            ($k: expr, $inc: expr, $i: expr) => {
                $i += 1;
                $k = ($k + $inc) % 16;
            };
        }

        let mut iter = 0;
        let (mut k, inc) = x[0];
        sixteen!(f, a, b, c, d, k, s[0][0], iter);
        backtrack!(f, a, b, c, d, k, s[0][0], iter);
        increment!(k, inc, iter);
        sixteen!(f, d, a, b, c, k, s[0][1], iter);
        backtrack!(f, d, a, b, c, k, s[0][1], iter);
        increment!(k, inc, iter);
        sixteen!(f, c, d, a, b, k, s[0][2], iter);
        backtrack!(f, c, d, a, b, k, s[0][2], iter);
        increment!(k, inc, iter);
        sixteen!(f, b, c, d, a, k, s[0][3], iter);
        backtrack!(f, b, c, d, a, k, s[0][3], iter);
        increment!(k, inc, iter);
        sixteen!(f, a, b, c, d, k, s[0][0], iter);
        backtrack!(f, a, b, c, d, k, s[0][0], iter);
        increment!(k, inc, iter);
        sixteen!(f, d, a, b, c, k, s[0][1], iter);
        backtrack!(f, d, a, b, c, k, s[0][1], iter);
        increment!(k, inc, iter);
        sixteen!(f, c, d, a, b, k, s[0][2], iter);
        backtrack!(f, c, d, a, b, k, s[0][2], iter);
        increment!(k, inc, iter);
        sixteen!(f, b, c, d, a, k, s[0][3], iter);
        backtrack!(f, b, c, d, a, k, s[0][3], iter);
        increment!(k, inc, iter);
        sixteen!(f, a, b, c, d, k, s[0][0], iter);
        backtrack!(f, a, b, c, d, k, s[0][0], iter);
        increment!(k, inc, iter);
        sixteen!(f, d, a, b, c, k, s[0][1], iter);
        backtrack!(f, d, a, b, c, k, s[0][1], iter);
        increment!(k, inc, iter);
        sixteen!(f, c, d, a, b, k, s[0][2], iter);
        backtrack!(f, c, d, a, b, k, s[0][2], iter);
        increment!(k, inc, iter);
        sixteen!(f, b, c, d, a, k, s[0][3], iter);
        backtrack!(f, b, c, d, a, k, s[0][3], iter);
        increment!(k, inc, iter);
        sixteen!(f, a, b, c, d, k, s[0][0], iter);
        backtrack!(f, a, b, c, d, k, s[0][0], iter);
        increment!(k, inc, iter);
        sixteen!(f, d, a, b, c, k, s[0][1], iter);
        backtrack!(f, d, a, b, c, k, s[0][1], iter);
        increment!(k, inc, iter);
        sixteen!(f, c, d, a, b, k, s[0][2], iter);
        backtrack!(f, c, d, a, b, k, s[0][2], iter);
        increment!(k, inc, iter);
        sixteen!(f, b, c, d, a, k, s[0][3], iter);
        backtrack!(f, b, c, d, a, k, s[0][3], iter);
        increment!(k, inc, iter);

        for (round, func) in [g, h, i].iter().enumerate() {
            let (mut k, inc) = x[round + 1];
            for _ in 0..4 {
                sixteen!(func, a, b, c, d, k, s[round][0], iter);
                increment!(k, inc, iter);
                sixteen!(func, d, a, b, c, k, s[round][1], iter);
                increment!(k, inc, iter);
                sixteen!(func, c, d, a, b, k, s[round][2], iter);
                increment!(k, inc, iter);
                sixteen!(func, b, c, d, a, k, s[round][3], iter);
                increment!(k, inc, iter);
            }
        }

        state
    }

    fn log_data(counter: Arc<AtomicU64>, found: Arc<AtomicBool>) {
        thread::spawn(move || {
            while !found.load(Ordering::Relaxed) {
                thread::sleep(Duration::from_secs(60));
                let c = counter.load(Ordering::Relaxed);
                println!(
                    "{} -> Progress: {} / {}, ({:.6}%)",
                    chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                    c,
                    1_u64 << 37,
                    (c as f64 / (1_u64 << 37) as f64) * 100.0,
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

        Self::log_data(counter.clone(), found.clone());

        (0..(1_u64 << 50)).into_par_iter().for_each(|_| {
            let mut m1 = Self::random_message();
            let mut h = Self::process_message(&mut m1, iv_0.clone());
            h += iv_0;

            let m1_prim: Vec<u32> = m1
                .iter()
                .zip(self.delta_m1.iter())
                .map(|(&x, &y)| {
                    ((x as i64 + y) % (1 << 32)) as u32
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
                        m1_prim,
                    );
                    *result.lock().unwrap() = m1;
                    found.store(true, Ordering::Relaxed);
                    return;
                } else {
                    println!(
                        "{} -> False positive",
                        chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                    );
                }
            }
            counter.fetch_add(1, Ordering::Relaxed);
        });

        *result.lock().unwrap()
    }
}
