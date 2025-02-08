

use std::collections::HashMap;
use std::fmt;
use std::str;

const MAX_INT: i32 = 94;
const ZERO: char = '0';

struct MoovIoAchIATBatchHeader {
    batch_number: i32,
    // ... other fields ...
}

struct MoovIoAchConverters;

impl MoovIoAchIATBatchHeader {
    fn batch_number_field(&self) -> String {
        MoovIoAchConverters::numeric_field(self.batch_number, 7)
    }
}

impl MoovIoAchConverters {
    fn numeric_field(n: i32, max: u32) -> String {
        let s = n.to_string();
        let l = s.len() as u32;
        if l > max {
            return s[(l - max) as usize..].to_string();
        } else {
            let m = max - l;
            let pad = get_pad_string(m as usize);
            return pad.repeat(m as usize) + &s;
        }
    }
}

fn get_pad_string(n: usize) -> String {
    let mut out = String::new();
    for _ in 0..=n {
        out.push('0');
    }
    out
}

impl fmt::Display for MoovIoAchIATBatchHeader {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "BatchNumber: {}, ...",
            self.batch_number_field()
        )
    }
}

