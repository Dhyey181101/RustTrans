

use std::collections::HashMap;
use std::fmt;
use std::str;

const MAX: usize = 94;
const ZERO: &str = "0";

struct MoovIoAchBatchControl {
    total_credit: i32,
    converters: Box<MoovIoAchConverters>,
}

struct MoovIoAchConverters {
    hash_map: HashMap<usize, String>,
}

impl MoovIoAchBatchControl {
    fn total_credit_entry_dollar_amount_field(&self) -> String {
        self.converters.numeric_field(self.total_credit, 12)
    }
}

impl MoovIoAchConverters {
    fn new() -> MoovIoAchConverters {
        let mut out = HashMap::new();
        for i in 0..=MAX {
            out.insert(i, "0".repeat(i));
        }
        MoovIoAchConverters { hash_map: out }
    }

    fn numeric_field(&self, n: i32, max: u32) -> String {
        let s = n.to_string();
        let l = s.len() as u32;
        if l > max {
            return s[(l - max) as usize..].to_string();
        } else {
            let m = max - l;
            let pad = &self.hash_map[&(m as usize)];
            return pad.clone() + &s;
        }
    }
}

// Added this
impl fmt::Debug for MoovIoAchConverters {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "MoovIoAchConverters {{ hash_map: {:?} }}", self.hash_map)
    }
}

impl fmt::Display for MoovIoAchBatchControl {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "total_credit: {}, converters: {:?}",
            self.total_credit, self.converters
        )
    }
}

fn main() {
    let batch_control = MoovIoAchBatchControl {
        total_credit: 123456,
        converters: Box::new(MoovIoAchConverters::new()),
    };
    println!(
        "{}", batch_control.total_credit_entry_dollar_amount_field()
    );
}

