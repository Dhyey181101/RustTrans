
use std::collections::HashMap;
use std::str::FromStr;
use std::string::ToString;

#[derive(Debug)]
pub struct EntryDetail {
    pub individual_name: String,
}

impl EntryDetail {
    pub fn set_catx_addenda_records(&mut self, i: usize) {
        let count = i.to_string();
        if self.individual_name.len() > 4 {
            self.individual_name = count + &self.individual_name[4..];
        } else {
            self.individual_name = count + &" ".repeat(16) + "  ";
        }
    }
}

#[derive(Debug)]
pub struct Converters {}

impl Converters {
    pub fn numeric_field(&self, n: usize, max: u32) -> String {
        let s = n.to_string();
        if s.len() as u32 > max {
            return s[(s.len() - max as usize)..].to_string();
        } else {
            let m = max - s.len() as u32;
            let pad = string_zeros(m as usize);
            return pad + &s;
        }
    }

    pub fn alpha_field(&self, s: &str, max: u32) -> String {
        let ln = s.chars().count() as u32;
        if ln > max {
            return s[..max as usize].to_string();
        }

        let m = max - ln;
        let pad = string_spaces(m as usize);
        return s.to_string() + &pad;
    }
}

fn string_zeros(max: usize) -> String {
    let mut out = HashMap::new();
    for i in 0..max {
        out.insert(i, "0".repeat(i));
    }
    out[&max].to_string()
}

fn string_spaces(max: usize) -> String {
    let mut out = HashMap::new();
    for i in 0..max {
        out.insert(i, " ".repeat(i));
    }
    out[&max].to_string()
}
