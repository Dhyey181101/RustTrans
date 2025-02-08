
use std::collections::HashMap;
use std::str::FromStr;
use std::string::ToString;

#[derive(Debug)]
pub struct MoovIoAchEntryDetail {
    pub individual_name: String,
}

impl MoovIoAchEntryDetail {
    pub fn set_catx_addenda_records(&mut self, i: usize) {
        let count = i.to_string();
        if self.individual_name.len() > 4 {
            self.individual_name = count + &self.individual_name[4..];
        } else {
            self.individual_name = count + &" ".repeat(16) + "  ";
        }
    }
}

pub struct MoovIoAchConverters {}

impl MoovIoAchConverters {
    pub fn numeric_field(&self, n: usize, max: u32) -> String {
        let s = n.to_string();
        if s.len() as u32 > max {
            return s[(s.len() - max as usize)..].to_string();
        } else {
            let m = max - s.len() as u32;
            let pad = moov_io_ach_string_zeros(m as usize);
            return pad + &s;
        }
    }

    pub fn alpha_field(&self, s: &str, max: u32) -> String {
        let ln = s.chars().count();
        if ln > max as usize {
            return s[..max as usize].to_string();
        }

        let m = max - ln as u32;
        let pad = moov_io_ach_space_zeros(m as usize);
        return s.to_string() + &pad;
    }
}

fn moov_io_ach_populate_map(max: usize, zero: &str) -> HashMap<usize, String> {
    let mut out = HashMap::new();
    for i in 0..max {
        out.insert(i, zero.repeat(i));
    }
    out
}

fn moov_io_ach_string_zeros(m: usize) -> String {
    let mut out = String::new();
    for _ in 0..m {
        out.push('0');
    }
    out
}

fn moov_io_ach_space_zeros(m: usize) -> String {
    let mut out = String::new();
    for _ in 0..m {
        out.push(' ');
    }
    out
}
