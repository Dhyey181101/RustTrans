

use std::fmt;
use std::collections::HashMap;
use std::boxed::Box;

struct MoovIoAchEntryDetail {
    amount: i32,
    // ... other fields elided ...
    converters: Box<MoovIoAchConverters>,
}

struct MoovIoAchConverters;

impl MoovIoAchEntryDetail {
    fn amount_field(&self) -> String {
        self.converters.numeric_field(self.amount, 10)
    }
}

impl MoovIoAchConverters {
    fn numeric_field(&self, n: i32, max: u32) -> String {
        let s = n.to_string();
        let l = s.len() as u32;
        if l > max {
            let start = (l-max) as usize;
            let mut result = String::new();
            for (i, c) in s.chars().enumerate() {
                if i >= start {
                    result.push(c);
                }
            }
            result
        } else {
            let m = max - l;
            let pad = get_zeros(m as usize);
            pad + &s
        }
    }
}

fn get_zeros(n: usize) -> String {
    let mut out = String::new();
    for _ in 0..n {
        out.push('0');
    }
    out
}

fn moov_io_ach_populate_map(max: i32, zero: char) -> HashMap<i32, String> {
    let mut out = HashMap::new();
    for i in 0..max {
        let zeros = (0..i).map(|_| zero).collect::<Vec<char>>().into_iter().collect::<String>();
        out.insert(i, zeros);
    }
    out
}

