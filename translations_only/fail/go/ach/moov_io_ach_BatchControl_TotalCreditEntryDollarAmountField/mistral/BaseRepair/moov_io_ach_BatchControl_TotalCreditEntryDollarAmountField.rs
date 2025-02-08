

use std::collections::HashMap;
use std::fmt;

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
        for i in 0..MAX {
            out.insert(i, "0".repeat(i));
        }
        MoovIoAchConverters { hash_map: out }
    }

    fn numeric_field(&self, n: i32, max: u32) -> String {
        let mut result = String::new();
        let mut number = n;
        let mut negative = number < 0;

        if negative {
            number = -number;
        }

        while number > 0 {
            let rem = number % 10;
            number /= 10;
            result.push(ZERO.chars().nth(rem as usize).unwrap());
        }

        if negative {
            result.insert(0, '-');
        }

        if result.len() > max as usize {
            result = format!("{}...", &result[..max as usize - 3]);
        }

        result
    }
}

