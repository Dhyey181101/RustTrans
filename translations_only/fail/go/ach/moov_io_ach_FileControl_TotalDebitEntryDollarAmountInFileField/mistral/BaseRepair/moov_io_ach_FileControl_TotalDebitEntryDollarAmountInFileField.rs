

use std::collections::HashMap;
use std::fmt;
use std::string::String;

const ZERO: char = '0';

struct MoovIoAchFileControl {
    total_debit: i32,
    converters: MoovIoAchConverters,
}

struct MoovIoAchConverters;

impl MoovIoAchFileControl {
    fn total_debit_entry_dollar_amount_in_file_field(&self) -> String {
        self.converters.numeric_field(self.total_debit, 12)
    }
}

impl MoovIoAchConverters {
    fn numeric_field(&self, n: i32, max: u32) -> String {
        let s = n.to_string();
        let l = s.len() as u32;
        if l > max {
            s[(l - max) as usize..].to_string()
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

fn moov_io_ach_populate_map(max: usize, zero: char) -> HashMap<usize, String> {
    let mut out = HashMap::new();
    for i in 0..max {
        let mut s = String::new();
        for _ in 0..i {
            s.push(zero);
        }
        out.insert(i, s);
    }
    out
}

impl fmt::Display for MoovIoAchFileControl {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "total_debit: {}\n",
            self.total_debit_entry_dollar_amount_in_file_field()
        )
    }
}

fn main() {
    let mut fc = MoovIoAchFileControl {
        total_debit: 123456,
        converters: MoovIoAchConverters {},
    };
    println!("{}", fc);
}

