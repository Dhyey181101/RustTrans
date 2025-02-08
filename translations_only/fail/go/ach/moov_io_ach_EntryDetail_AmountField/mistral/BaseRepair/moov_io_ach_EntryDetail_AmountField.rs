

use std::fmt;
use std::collections::HashMap;
use std::iter;

struct MoovIoAchEntryDetail {
    amount: i32,
}

struct MoovIoAchConverters;

impl MoovIoAchEntryDetail {
    fn amount_field(&self) -> String {
        MoovIoAchConverters::numeric_field(&self.amount, 10)
    }
}

impl MoovIoAchConverters {
    fn numeric_field(n: &i32, max: u32) -> String {
        let s = n.to_string();
        let l = s.len() as u32;
        if l > max {
            s[(l-max) as usize..].to_string()
        } else {
            let m = max - l;
            let pad = get_zeros(m as usize);
            pad + &s
        }
    }
}

fn get_zeros(n: usize) -> String {
    iter::repeat('0').take(n).collect()
}

fn populate_map(max: i32, zero: char) -> HashMap<i32, String> {
    let mut out = HashMap::new();
    for i in 0..max {
        let zeros = get_zeros(i as usize);
        out.insert(i, zeros);
    }
    out
}

impl fmt::Display for MoovIoAchEntryDetail {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.amount_field())
    }
}

fn main() {
    let mut map = populate_map(94, '0');
    let entry = MoovIoAchEntryDetail { amount: 12345 };
    println!("{}", entry);
}

