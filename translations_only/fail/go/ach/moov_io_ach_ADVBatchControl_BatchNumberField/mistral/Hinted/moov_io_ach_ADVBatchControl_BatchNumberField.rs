

use std::collections::HashMap;
use std::fmt;
use std::str;

const ZEROS: &str = "0";

struct MoovIoAchAdvBatchControl {
    batch_number: i32,
    // ... other fields and methods ...
}

impl MoovIoAchAdvBatchControl {
    fn batch_number_field(&self) -> String {
        MoovIoAchConverters::new().numeric_field(self.batch_number, 7)
    }
}

struct MoovIoAchConverters;

impl MoovIoAchConverters {
    fn new() -> MoovIoAchConverters {
        MoovIoAchConverters {}
    }

    fn numeric_field(&self, n: i32, max: u32) -> String {
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

impl fmt::Display for MoovIoAchAdvBatchControl {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{{
            batch_number: {},
            // ... other fields ...
        }}",
            self.batch_number
        )
    }
}

fn main() {
    let batch_control = MoovIoAchAdvBatchControl {
        batch_number: 12345,
        // ... other fields ...
    };
    println!("{}", batch_control.batch_number_field());
}

