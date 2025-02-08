

use std::collections::HashMap;
use std::fmt;
use std::string::String;

const ZEROS: &str = "0";

struct MoovIoAchFileControl {
    total_credit: i32,
    converters: Box<MoovIoAchConverters>,
}

struct MoovIoAchConverters {
    hash_map: HashMap<usize, String>,
}

impl MoovIoAchFileControl {
    fn total_credit_entry_dollar_amount_in_file_field(&self) -> String {
        self.converters.numeric_field(self.total_credit, 12)
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
            let pad = self.hash_map[&(m as usize)].clone();
            return format!("{}{}", pad, &s);
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

impl MoovIoAchConverters {
    fn new() -> MoovIoAchConverters {
        let mut map = HashMap::new();
        for i in 0..=12 {
            map.insert(i, get_pad_string(i));
        }
        MoovIoAchConverters { hash_map: map }
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
        converters: Box::new(MoovIoAchConverters::new()),
    };
    println!("{}", fc);
}

