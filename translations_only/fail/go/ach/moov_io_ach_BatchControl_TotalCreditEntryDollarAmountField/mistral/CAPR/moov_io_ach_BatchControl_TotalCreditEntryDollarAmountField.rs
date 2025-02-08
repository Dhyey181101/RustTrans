

use std::collections::HashMap;
use std::fmt;

const ZEROS: &str = "0";

struct MoovIoAchBatchControl {
    total_credit: i32,
    converters: Box<MoovIoAchConverters>,
}

struct MoovIoAchConverters {
    zeros: HashMap<usize, String>,
}

impl MoovIoAchConverters {
    fn new() -> Self {
        MoovIoAchConverters {
            zeros: get_zeros(),
        }
    }

    fn numeric_field(&self, n: i32, max: u32) -> String {
        let s = n.to_string();
        let l = s.len() as u32;
        if l > max {
            let start = l - max;
            let mut s = s.chars();
            s.nth(start as usize).unwrap().to_string() + &s.collect::<String>()
        } else {
            let m = max - l;
            self.zeros[&(m as usize)].clone() + &s
        }
    }
}

fn get_zeros() -> HashMap<usize, String> {
    let mut out = HashMap::new();
    for i in 0..=12 {
        out.insert(i, "0".repeat(i));
    }
    out
}

impl fmt::Display for MoovIoAchBatchControl {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "total_credit: {}",
            self.converters.numeric_field(self.total_credit, 12)
        )
    }
}

fn main() {
    let bc = MoovIoAchBatchControl {
        total_credit: 123456,
        converters: Box::new(MoovIoAchConverters::new()),
    };
    println!("{}", bc);
}

