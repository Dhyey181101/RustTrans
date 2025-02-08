

use std::collections::HashMap;
use std::fmt;
use std::str;

struct MoovIoAchAdvEntryDetail {
    amount: i32,
}

impl MoovIoAchAdvEntryDetail {
    fn amount_field(&self) -> String {
        MoovIoAchConverters::new().numeric_field(self.amount, 12)
    }
}

struct MoovIoAchConverters;

impl MoovIoAchConverters {
    fn new() -> Self {
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
    "0".repeat(n)
}

fn moov_io_ach_populate_map(max: usize, zero: &str) -> HashMap<usize, String> {
    let mut out = HashMap::new();
    for i in 0..max {
        out.insert(i, get_pad_string(i).repeat(i));
    }
    return out;
}

impl fmt::Display for MoovIoAchAdvEntryDetail {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.amount_field())
    }
}

fn main() {
    let mut map = moov_io_ach_populate_map(94, "0");
    let mut converters = MoovIoAchConverters::new();
    let mut ed = Box::new(MoovIoAchAdvEntryDetail { amount: 63 });
    println!("{}", ed);
    ed.amount = 771751983;
    println!("{}", ed);
    ed.amount = 5;
    println!("{}", ed);
    ed.amount = 0;
    println!("{}", ed);
}

