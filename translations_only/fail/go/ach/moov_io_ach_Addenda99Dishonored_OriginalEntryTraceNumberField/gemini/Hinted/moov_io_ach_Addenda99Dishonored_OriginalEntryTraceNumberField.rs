
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug)]
pub struct MoovIoAchAddenda99Dishonored {
    pub original_entry_trace_number: String,
}

impl MoovIoAchAddenda99Dishonored {
    pub fn original_entry_trace_number_field(&self) -> &str {
        &self.original_entry_trace_number
    }
}

pub struct MoovIoAchConverters {}

impl MoovIoAchConverters {
    pub fn string_field(&self, s: &str, max: usize) -> String {
        let ln = s.chars().count();
        if ln > max {
            return s[..max].to_string();
        }

        let m = max - ln;
        let pad = moov_io_ach_string_zeros(m);
        pad + s
    }
}

fn moov_io_ach_string_zeros(m: usize) -> String {
    let mut out = String::with_capacity(m);
    for _ in 0..m {
        out.push('0');
    }
    out
}

fn moov_io_ach_populate_map(max: usize, zero: &str) -> HashMap<usize, String> {
    let mut out = HashMap::with_capacity(max);
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
        assert_eq!(c.string_field("1234567890", 5), "12345");
        assert_eq!(c.string_field("12345", 10), "1234500000");
    }

    #[test]
    fn test_moov_io_ach_populate_map() {
        let out = moov_io_ach_populate_map(10, "0");
        assert_eq!(out.get(&0), Some(&"".to_string()));
        assert_eq!(out.get(&1), Some(&"0".to_string()));
        assert_eq!(out.get(&9), Some(&"000000000".to_string()));
    }
}
