use rand::{self, distr::Alphanumeric, Rng};
use md5;
use std::io::Write;

mod lib;
use lib::md5::Md5;

fn main() -> std::io::Result<()> {
    let iters = 10000;
    let mut avg = 0_f64;
    let s_len = 1000;
    let mut file = std::io::BufWriter::new(std::fs::File::create("benchmark_md5.csv")?);
    file.write_all(b"length,my_md5_ns,their_md5_ns\n")?;

    for n in 0..s_len {
        print!("\rBenchmarking {}/{}", n, s_len);
        let s: String = rand::rng()
            .sample_iter(Alphanumeric)
            .take(s_len)
            .map(char::from)
            .collect();
        let my = Md5::new(&s);
        let their = md5::compute(&s);
        assert_eq!(my.to_str(), format!("{:x}", their));

        let mut avg1 = 0_f64;
        for _ in 0..iters {
            let _hash: Md5;
            let start = std::time::Instant::now();
            _hash = Md5::new(&s);
            let duration = start.elapsed();
            avg1 += duration.as_nanos() as f64;
        }
        // println!("Average time for my md5 for n = {}: {:?}", n, avg / iters as f64);
        let mut avg2 = 0_f64;
        for _ in 0..iters {
            let _hash: md5::Digest;
            let start = std::time::Instant::now();
            _hash = md5::compute(&s);
            let duration = start.elapsed();
            avg2 += duration.as_nanos() as f64;
        }
        // println!("Average time for theirs md5 for n = {}: {:?}", n, avg / iters as f64);
        file.write_all(format!("{},{},{}\n", n, avg1 / iters as f64, avg2 / iters as f64).as_bytes())?;
        avg += avg1 / avg2;
    }

    file.flush()?;
    println!("\rOverall average time ratio (mine/theirs): {:?}", avg / s_len as f64);

    Ok(())
}
