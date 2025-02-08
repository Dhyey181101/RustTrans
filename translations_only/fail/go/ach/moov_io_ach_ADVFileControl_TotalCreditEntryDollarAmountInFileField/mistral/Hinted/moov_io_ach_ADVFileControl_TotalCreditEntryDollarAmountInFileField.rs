

use std::collections::HashMap;
use std::fmt;
use std::str;

const ZERO: &str = "0";

struct MoovIoAchADVFileControl {
    total_credit: i32,
    converters: Box<MoovIoAchConverters>,
}

struct MoovIoAchConverters;

impl MoovIoAchADVFileControl {
    fn total_credit_entry_dollar_amount_in_file_field(&self) -> String {
        self.converters.numeric_field(self.total_credit, 20)
    }
}

impl MoovIoAchConverters {
    fn numeric_field(&self, n: i32, max: u32) -> String {
        let s = n.to_string();
        let l = s.len() as u32;
        if l > max {
            return s[(l - max) as usize..].to_string();
        } else {
            let m = max - l;
            let pad = MoovIoAchConverters::get_pad(m as usize);
            return pad.to_string() + &s;
        }
    }

    fn get_pad(n: usize) -> String {
        let mut pad = String::new();
        for _ in 0..n {
            pad.push_str(ZERO);
        }
        pad
    }
}

impl fmt::Display for MoovIoAchADVFileControl {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{{total_credit:{}}}",
            self.total_credit_entry_dollar_amount_in_file_field()
        )
    }
}

fn main() {
    let fc = MoovIoAchADVFileControl {
        total_credit: 123456,
        converters: Box::new(MoovIoAchConverters {}),
    };
    println!("{}", fc);
}

