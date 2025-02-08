

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
        }
        let m = (max - l) as usize;
        let pad = "0".repeat(m);
        format!("{}{}", pad, s)
    }

    fn alpha_field(&self, s: &str, max: u32) -> String {
        let ln = s.len() as u32;
        if ln > max {
            return s[..(max as usize)].to_string();
        }
        let m = (max - ln) as usize;
        let pad = " ".repeat(m);
        format!("{}{}", s, pad)
    }
}

struct MoovIoAchEntryDetail {
    individual_name: String,
}

impl MoovIoAchEntryDetail {
    fn set_catx_addenda_records(&mut self, i: i32) {
        let count = self.numeric_field(i, 4);
        let current = &self.individual_name;
        if current.len() > 4 {
            self.individual_name = format!("{}{}", count, &current[4..]);
        } else {
            self.individual_name = format!("{}{}{}", count, " ".repeat(16), "  ");
        }
    }

    fn numeric_field(&self, n: i32, max: u32) -> String {
        MoovIoAchConverters.numeric_field(n, max)
    }
}

impl fmt::Display for MoovIoAchEntryDetail {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.individual_name)
    }
}

fn main() {
    let mut ed = MoovIoAchEntryDetail {
        individual_name: "John Doe".to_string(),
    };
    ed.set_catx_addenda_records(1234);
    println!("{}", ed);
}

