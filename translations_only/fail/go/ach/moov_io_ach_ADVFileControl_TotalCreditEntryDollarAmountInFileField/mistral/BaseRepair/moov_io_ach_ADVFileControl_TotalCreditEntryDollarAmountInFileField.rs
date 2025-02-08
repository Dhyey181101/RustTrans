

use std::collections::HashMap;
use std::fmt;

const ZEROS: &str = "0";

struct MoovIoAchConverters;

impl MoovIoAchConverters {
    fn numeric_field(&self, n: i32, max: usize) -> String {
        let s = n.to_string();
        let l = s.len();
        if l > max {
            s[l - max..].to_string()
        } else {
            let m = max - l;
            let pad = get_pad(m);
            pad + &s
        }
    }
}

fn get_pad(n: usize) -> String {
    let mut out = HashMap::new();
    for i in 0..=n {
        out.insert(i, "0".repeat(i));
    }
    out[&n].clone()
}

struct MoovIoAchAdvFileControl {
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
            "total_credit: {}\n",
            self.total_credit_entry_dollar_amount_in_file_field()
        )
    }
}

fn main() {
    let a = MoovIoAchAdvFileControl {
        total_credit: 123456,
        moov_io_ach_converters: MoovIoAchConverters,
    };
    println!("{}", a);
}

