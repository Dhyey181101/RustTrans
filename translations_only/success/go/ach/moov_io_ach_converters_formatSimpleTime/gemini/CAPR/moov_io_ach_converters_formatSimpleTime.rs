
use std::collections::HashMap;
use std::string::String;

pub struct MoovIoAchConverters {}

impl MoovIoAchConverters {
    pub fn format_simple_time(&self, s: &str) -> String {
        if s.is_empty() {
            return self.string_field(s, 4);
        }
        s.to_string()
    }

    pub fn string_field(&self, s: &str, max: u32) -> String {
        let ln = s.chars().count() as u32;
        if ln > max {
            return s[..max as usize].to_string();
        }

        let m = max - ln;
        let pad = moov_io_ach_populate_map(m as usize, "0");
        pad + s
    }
}

fn moov_io_ach_populate_map(max: usize, zero: &str) -> String {
    let mut out = HashMap::new();
    for i in 0..max {
        out.insert(i, zero.repeat(i));
    }
    out.get(&max).unwrap().clone()
}
