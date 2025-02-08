

use std::collections::HashMap;
use std::fmt;
use std::string::String;

const ZERO: char = '0';

struct MoovIoAchFileControl {
    total_credit: i32,
    converters: Box<MoovIoAchConverters>,
}

struct MoovIoAchConverters {
    pad: String,
}

impl MoovIoAchFileControl {
    fn total_credit_entry_dollar_amount_in_file_field(&self) -> String {
        self.converters.numeric_field(self.total_credit, 12)
    }
}

impl MoovIoAchConverters {
    fn new() -> Self {
        MoovIoAchConverters {
            pad: String::new(),
        }
    }

    fn numeric_field(&self, n: i32, max: u32) -> String {
        let s = n.to_string();
        let l = s.len() as u32;
        if l > max {
            s[(l - max) as usize..].to_string()
        } else {
            let m = max - l;
            self.pad.clone() + &s
        }
    }
}

fn moov_io_ach_populate_map(max: i32, zero: char) -> HashMap<i32, String> {
    let mut out = HashMap::new();
    for i in 0..max {
        let zero_str = (0..i).map(|_| zero).collect::<String>();
        out.insert(i, zero_str);
    }
    out
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
    let mut fc = MoovIoAchFileControl {
        total_credit: 123456,
        converters: Box::new(MoovIoAchConverters::new()),
    };
    println!("{}", fc);
}

