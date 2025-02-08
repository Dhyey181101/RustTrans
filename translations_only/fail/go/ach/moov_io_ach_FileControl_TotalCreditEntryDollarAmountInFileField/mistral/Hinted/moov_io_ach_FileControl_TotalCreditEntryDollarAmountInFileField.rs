

use std::collections::HashMap;
use std::fmt;
use std::string::String;

struct MoovIoAchConverters;

impl MoovIoAchConverters {
    fn numeric_field(&self, n: i32, max: u32) -> String {
        let s = n.to_string();
        let l = s.len() as u32;
        if l > max {
            return s[(l - max) as usize..].to_string();
        } else {
            let m = max - l;
            let pad = MoovIoAchFileControl::get_zeros(m as usize);
            return pad + &s;
        }
    }
}

struct MoovIoAchFileControl {
    total_credit: i32,
    moov_io_ach_converters: MoovIoAchConverters,
}

impl MoovIoAchFileControl {
    fn total_credit_entry_dollar_amount_in_file_field(&self) -> String {
        self.numeric_field(self.total_credit, 12)
    }

    fn numeric_field(&self, n: i32, max: u32) -> String {
        self.moov_io_ach_converters.numeric_field(n, max)
    }

    fn get_zeros(n: usize) -> String {
        let mut out = HashMap::new();
        for i in 0..=n {
            out.insert(i, "0".repeat(i));
        }
        out[&n].clone()
    }
}

impl fmt::Display for MoovIoAchFileControl {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "total_credit: {}",
            self.total_credit_entry_dollar_amount_in_file_field()
        )
    }
}

fn main() {
    let fc = MoovIoAchFileControl {
        total_credit: 123456,
        moov_io_ach_converters: MoovIoAchConverters {},
    };
    println!("{}", fc);
}

