

use std::collections::HashMap;
use std::fmt;
use std::str;

const ZERO: char = '0';

struct MoovIoAchFileControl {
    id: String,
    batch_count: i32,
    block_count: i32,
    entry_addenda_count: i32,
    entry_hash: i32,
    total_debit_entry_dollar_amount_in_file: f64,
    total_credit_entry_dollar_amount_in_file: f64,
    validator: (),
    converters: Box<MoovIoAchConverters>,
}

struct MoovIoAchConverters;

impl MoovIoAchFileControl {
    fn entry_hash_field(&self) -> String {
        self.converters.numeric_field(self.entry_hash, 10)
    }
}

impl MoovIoAchConverters {
    fn numeric_field(&self, n: i32, max: u32) -> String {
        let s = n.to_string();
        let l = s.len() as u32;
        if l > max {
            let start = l - max;
            let s = &s[start as usize..];
            s.to_string()
        } else {
            let m = max - l;
            let pad = get_pad(m as usize);
            pad + &s
        }
    }
}

fn get_pad(n: usize) -> String {
    let mut pad = String::new();
    for _ in 0..n {
        pad.push(ZERO);
    }
    pad
}

