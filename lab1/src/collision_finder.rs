use std::sync::{atomic::{AtomicBool, AtomicU64, Ordering}, Arc, Mutex};
use std::thread;
use std::time::Duration;
use rayon::prelude::*;

use rand::{self, rng, Rng};

use crate::{conditions::Condition, md5::Md5, state::State, consts};
use crate::bit_functions::*;

pub struct CollisionFinder {
    m0: [u32; 16],
    m0_prim: [u32; 16],
    m1: [u32; 16],
    conditions: Vec<Vec<Condition>>,
}

impl CollisionFinder {
    pub fn new(m0: [u32; 16], m0_prim: [u32; 16], conditions: Vec<Vec<Condition>>) -> Self {
        Self { m0, m0_prim, m1: Self::random_message(), conditions }
    }

    fn random_message() -> [u32; 16] {
        let mut rng = rng();
        let mut m1 = [0_u32; 16];
        for cell in &mut m1 {
            *cell = rng.random();
        }
        m1
    }

    fn modify_bit(&self, state: &mut State, step: usize) {
        let aplicable_conds = &self.conditions[step];
        if aplicable_conds.is_empty() {
            return;
        }

        for cond in aplicable_conds {
            let prev = match cond.typ {
                0 | 1 => 0,
                2 | 4 => 1,
                3 | 5 => 2,
                _ => unreachable!(),
            };
            let copy_value = match (step - prev) % 4 {
                0 => state.a,
                1 => state.d,
                2 => state.c,
                3 => state.b,
                _ => unreachable!(),
            };
            let value = match step % 4 {
                0 => &mut state.a,
                1 => &mut state.d,
                2 => &mut state.c,
                3 => &mut state.b,
                _ => unreachable!()
            };
            match cond.typ {
                0 => *value &= !(1 << cond.bit),
                1 => *value |= 1 << cond.bit,
                2 | 3 => *value = (*value & !(1 << cond.bit)) | (copy_value & (1 << cond.bit)),
                4 | 5 => *value = (*value & !(1 << cond.bit)) | (!copy_value & (1 << cond.bit)),
                _ => unreachable!(),
            };
        }
    }

    fn process_message(&self, m1_prim: &mut [u32; 16], mut state: State) -> State {
        let s = &consts::S;
        let t = &consts::T;
        let x = &consts::X_INDEX_START;

        macro_rules! sixteen {
            ($func: ident, $a: ident, $b: ident, $c: ident, $d: ident, $k: expr, $inc: expr, $s: expr, $i: expr) => {
                state.$a = (state.$a
                        .wrapping_add($func(state.$b, state.$c, state.$d))
                        .wrapping_add(m1_prim[$k])
                        .wrapping_add(t[$i]))
                    .rotate_left($s as u32)
                    .wrapping_add(state.$b);

                self.modify_bit(&mut state, $i);
                m1_prim[$k] = state.$a
                    .wrapping_sub(state.$b)
                    .rotate_right($s as u32)
                    .wrapping_sub(state.$a)
                    .wrapping_sub($func(state.$b, state.$c, state.$d));

                $i += 1;
                $k = ($k + $inc) % 16;
            };
        }

        let mut iter = 0;

        for (round, func) in [f,g,h,i].iter().enumerate() {
            let (mut k, inc) = x[round];
            for _ in 0..4 {
                sixteen!(func, a, b, c, d, k, inc, s[round][0], iter);
                sixteen!(func, d, a, b, c, k, inc, s[round][1], iter);
                sixteen!(func, c, d, a, b, k, inc, s[round][2], iter);
                sixteen!(func, b, c, d, a, k, inc, s[round][3], iter);
            }
        }

        state
    }

    pub fn find_collision(&self) -> ([u32; 16], [u32; 16], Md5) {
        let iv_0 = Md5::new_raw_block(&self.m0).get_state();
        let h = Md5::new_with_state_raw_block(&self.m1, iv_0);
        let hash = h.get_hash();
        let iv_0_prim = Md5::new_raw_block(&self.m0_prim).get_state();

        let found = Arc::new(AtomicBool::new(false));
        let result = Arc::new(Mutex::new([0u32; 16]));
        let counter = Arc::new(AtomicU64::new(0));

        {
            let counter = Arc::clone(&counter);
            let found = Arc::clone(&found);
            thread::spawn(move || {
                while !found.load(Ordering::Relaxed) {
                    thread::sleep(Duration::from_secs(5));
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

        (0..(1_u64 << 50)).into_par_iter().for_each(|_| {
            let mut m1_prim = Self::random_message();
            let mut h_prim = self.process_message(&mut m1_prim, iv_0_prim.clone());
            h_prim += iv_0_prim;

            if h_prim.get_hash() == hash {
                if Md5::new_with_state_raw_block(&self.m1, iv_0).get_hash() == Md5::new_with_state_raw_block(&m1_prim, iv_0_prim).get_hash() {
                    println!(
                        "{} -> Found!!! {:?}",
                        chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                        m1_prim,
                    );
                    *result.lock().unwrap() = m1_prim;
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

        (self.m1, *result.lock().unwrap(), h)
    }
}
