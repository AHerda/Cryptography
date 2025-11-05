use std::{fs::File, io::{BufRead, BufReader}};

#[derive(Debug, Clone)]
pub struct Condition {
    pub step: usize,
    pub bit: u32,
    pub typ: u32,
}

impl Condition {
    pub fn read_condtions_file() -> std::io::Result<Vec<Vec<Condition>>> {
        let f = File::open(&"cond.txt")?;
        let reader = BufReader::new(f);

        let mut conds = vec![vec![]; 65];

        for (line_no, line) in reader.lines().enumerate() {
            let line = line?.trim().to_string();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            let parts: Vec<_> = line.split_whitespace().collect();
            if parts.len() != 3 {
                println!("Wrong number of inputs in a line {}", line_no)
            }
            let step: usize = parts[0].parse().expect(&format!("Error parsing step in line {}", line_no));
            let bit: u32 = parts[1].parse().expect(&format!("Error parsing bit in line {}", line_no));
            let typ: u32 = parts[2].parse().expect(&format!("Error parsing type in line {}", line_no));
            conds[step].push(Condition { step, bit: bit - 1, typ });
        }

        Ok(conds)
    }
}
