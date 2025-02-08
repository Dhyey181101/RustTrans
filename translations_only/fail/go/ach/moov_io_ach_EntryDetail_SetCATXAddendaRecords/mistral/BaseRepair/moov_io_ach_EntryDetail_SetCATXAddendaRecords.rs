

use std::collections::HashMap;
use std::fmt;
use std::str;

const ZERO: &str = "0";
const SPACE: &str = " ";

struct EntryDetail {
    individual_name: String,
}

struct Converters;

impl EntryDetail {
    fn set_catx_addenda_records(&mut self, i: i32) {
        let count = self.numeric_field(i, 4);
        let current = &self.individual_name;
        if current.len() > 4 {
            self.individual_name = format!("{}{}", count, &current[4..]);
        } else {
            self.individual_name = format!("{}{}{}  ", count, " ".repeat(16), " ".repeat(2));
        }
    }

    fn numeric_field(&self, n: i32, max: u32) -> String {
        let s = n.to_string();
        let l = s.len() as u32;
        if l > max {
            return s[l as usize-max as usize..].to_string();
        }
        let m = (max - l) as usize;
        let pad = get_pad(m, ZERO);
        pad + &s
    }

    fn alpha_field(&self, s: &str, max: u32) -> String {
        let ln = s.len() as u32;
        if ln > max {
            return s[..max as usize].to_string();
        }
        let m = (max - ln) as usize;
        let pad = get_pad(m, SPACE);
        s.to_string() + &pad
    }
}

impl Converters {
    fn new() -> Converters {
        Converters {}
    }
}

fn get_pad(n: usize, pad_char: &str) -> String {
    let mut pad = HashMap::new();
    for i in 0..94 {
        pad.insert(i, "0".repeat(i));
    }
    pad[&n].clone()
}

impl fmt::Display for EntryDetail {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.individual_name)
    }
}

