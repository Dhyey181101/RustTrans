

use std::collections::HashMap;
use std::fmt;
use std::str;

struct MoovIoAchConverters;

impl MoovIoAchConverters {
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

struct MoovIoAchAdvBatchControl {
    total_debit_entry_dollar_amount: i32,
}

impl MoovIoAchAdvBatchControl {
    fn total_debit_entry_dollar_amount_field(&self) -> String {
        MoovIoAchConverters::numeric_field(&MoovIoAchConverters, self.total_debit_entry_dollar_amount, 20)
    }
}

fn get_pad_string(n: usize) -> String {
    "0".repeat(n)
}

fn moov_io_ach_populate_map(max: i32, zero: &str) -> HashMap<i32, String> {
    let mut out = HashMap::new();
    for i in 0..max {
        out.insert(i, get_pad_string(i as usize).repeat(i as usize));
    }
    out
}

impl fmt::Display for MoovIoAchAdvBatchControl {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "total_debit_entry_dollar_amount: {}",
            self.total_debit_entry_dollar_amount
        )
    }
}

fn main() {
    let mut moov_io_ach_string_zeros = moov_io_ach_populate_map(94, "0");
    let bc = Box::new(MoovIoAchAdvBatchControl {
        total_debit_entry_dollar_amount: 12345,
    });
    println!("{}", bc.total_debit_entry_dollar_amount_field());
}

