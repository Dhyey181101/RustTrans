

use std::collections::HashMap;
use std::fmt;
use std::string::String;

const ZERO: char = '0';

struct MoovIoAchConverters;

impl MoovIoAchConverters {
    fn numeric_field(&self, n: i32, max: usize) -> String {
        let s = n.to_string();
        let l = s.len();
        if l > max {
            s[l - max..].to_string()
        } else {
            let m = max - l;
            let pad = get_pad_string(m);
            pad + &s
        }
    }
}

fn get_pad_string(n: usize) -> String {
    let mut out = HashMap::new();
    for i in 0..=n {
        out.insert(i, "0".repeat(i));
    }
    out[&n].clone()
}

struct MoovIoAchAdvFileControl {
    total_debit: i32,
    total_credit: i32,
    moov_io_ach_converters: MoovIoAchConverters,
}

impl MoovIoAchAdvFileControl {
    fn total_credit_entry_dollar_amount_in_file_field(&self) -> String {
        self.moov_io_ach_converters.numeric_field(self.total_credit, 20)
    }
}

impl fmt::Display for MoovIoAchAdvFileControl {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "total_debit: {}, total_credit: {}",
            self.total_debit, self.total_credit
        )
    }
}

fn main() {
    let a = MoovIoAchAdvFileControl {
        total_debit: 123,
        total_credit: 456,
        moov_io_ach_converters: MoovIoAchConverters,
    };
    println!(
        "{}", a.total_credit_entry_dollar_amount_in_file_field()
    );
}

