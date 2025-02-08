

use std::collections::HashMap;
use std::fmt;
use std::str;

const ZEROS: &str = "0";

struct MoovIoAchConverters;

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
            let pad = get_zeros(m as usize);
            if let Some(p) = pad {
                p + &s
            } else {
                "0".repeat(m as usize) + &s
            }
        }
    }
}

fn get_zeros(n: usize) -> Option<String> {
    let mut map = HashMap::new();
    for i in 0..=n {
        map.insert(i, "0".repeat(i));
    }
    map.get(&n).cloned()
}

struct MoovIoAchBatchControl {
    total_debit: i32,
}

impl MoovIoAchBatchControl {
    fn total_debit_entry_dollar_amount_field(&self) -> String {
        MoovIoAchConverters.numeric_field(self.total_debit, 12)
    }
}

impl fmt::Display for MoovIoAchBatchControl {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "total_debit: {}\n",
            self.total_debit_entry_dollar_amount_field()
        )
    }
}

fn main() {
    let bc = MoovIoAchBatchControl { total_debit: 123456 };
    println!("{}", bc);
}

