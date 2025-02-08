

use std::collections::HashMap;
use std::fmt;
use std::str;

const SERVICE_CLASS_CODE_MIXED: i32 = 200;
const SERVICE_CLASS_CODE_CREDITS: i32 = 220;
const SERVICE_CLASS_CODE_DEBITS: i32 = 225;

struct MoovIoAchBatchControl {
    batch_number: i32,
    // ... other fields ...
    converters: MoovIoAchConverters,
}

impl MoovIoAchBatchControl {
    fn batch_number_field(&self) -> String {
        self.converters.numeric_field(self.batch_number, 7)
    }
}

struct MoovIoAchConverters;

impl MoovIoAchConverters {
    fn numeric_field(&self, n: i32, max: u32) -> String {
        let s = n.to_string();
        let l = s.len() as u32;
        if l > max {
            return s[(l - max) as usize..].to_string();
        } else {
            let m = max - l;
            let pad = get_pad_string(m as usize, "0");
            return pad + &s;
        }
    }
}

fn get_pad_string(n: usize, c: &str) -> String {
    let mut out = String::new();
    for _ in 0..n {
        out.push(c.chars().next().unwrap());
    }
    return out;
}

fn moov_io_ach_populate_map(max: i32, zero: &str) -> HashMap<i32, String> {
    let mut out = HashMap::new();
    for i in 0..max {
        let mut s = String::new();
        for _ in 0..i {
            s.push_str(zero);
        }
        out.insert(i, s);
    }
    return out;
}

// Implement Display trait for MoovIoAchBatchControl
impl fmt::Display for MoovIoAchBatchControl {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "MoovIoAchBatchControl [ batchNumber: {} ]",
            self.batch_number_field()
        )
    }
}

fn main() {
    let mut moov_io_ach_string_zeros = moov_io_ach_populate_map(94, "0");
    let bc = Box::new(MoovIoAchBatchControl {
        batch_number: 12345,
        // ... initialize other fields ...
        converters: MoovIoAchConverters {},
    });
    println!("Batch Number Field: {}", bc.batch_number_field());
}

