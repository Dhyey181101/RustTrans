
use std::collections::HashMap;
use std::fmt::Display;
use std::ops::Deref;
use std::str::FromStr;

#[derive(Debug)]
pub struct MoovIoAchAddenda99Contested {
    pub dishonored_return_trace_number: String,
}

impl MoovIoAchAddenda99Contested {
    pub fn dishonored_return_trace_number_field(&self) -> &str {
        &self.dishonored_return_trace_number
    }
}

#[derive(Debug)]
pub struct MoovIoAchConverters {}

impl MoovIoAchConverters {
    pub fn string_field(&self, s: &str, max: u32) -> String {
        let ln = s.chars().count() as u32;
        if ln > max {
            return s[..max as usize].to_string();
        }

        let m = max - ln;
        let pad = moov_io_ach_string_zeros(m as usize);
        format!("{}{}", pad, s)
    }
}

fn moov_io_ach_string_zeros(m: usize) -> String {
    let mut out = String::new();
    for _ in 0..m {
        out.push('0');
    }
    out
}

fn moov_io_ach_populate_map(max: usize, zero: &str) -> HashMap<usize, String> {
    let mut out = HashMap::new();
    for i in 0..max {
        out.insert(i, zero.repeat(i));
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_field() {
        let c = MoovIoAchConverters {};
        assert_eq!(c.string_field("12345", 5), "12345");
        assert_eq!(c.string_field("123456", 5), "12345");
        assert_eq!(c.string_field("1234567", 5), "01234");
    }

    #[test]
    fn test_moov_io_ach_populate_map() {
        let out = moov_io_ach_populate_map(5, "0");
        assert_eq!(out[&0], "");
        assert_eq!(out[&1], "0");
        assert_eq!(out[&2], "00");
        assert_eq!(out[&3], "000");
        assert_eq!(out[&4], "0000");
    }
}
