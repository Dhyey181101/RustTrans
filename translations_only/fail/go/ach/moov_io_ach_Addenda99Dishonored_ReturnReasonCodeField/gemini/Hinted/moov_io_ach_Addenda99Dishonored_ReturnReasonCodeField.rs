
use std::collections::HashMap;
use std::str::FromStr;
use std::string::ToString;

pub struct MoovIoAchAddenda99Dishonored {
    pub return_reason_code: String,
    pub addenda_information: String,
    pub trace_number: String,
}

pub struct MoovIoAchConverters {}

impl MoovIoAchAddenda99Dishonored {
    pub fn return_reason_code_field(&self) -> String {
        self.return_reason_code.clone()
    }
}

impl MoovIoAchConverters {
    pub fn string_field(&self, s: &str, max: u32) -> String {
        let ln = s.chars().count() as u32;
        if ln > max {
            return s[..max as usize].to_string();
        }

        let m = max - ln;
        let pad = moov_io_ach_string_zeros(m as usize);
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
        assert_eq!(c.string_field("123456", 5), "12345");
        assert_eq!(c.string_field("1234567890", 5), "12345");
    }

    #[test]
    fn test_moov_io_ach_populate_map() {
        let out = moov_io_ach_populate_map(5, "0");
        assert_eq!(out.get(&0), Some(&"".to_string()));
        assert_eq!(out.get(&1), Some(&"0".to_string()));
        assert_eq!(out.get(&2), Some(&"00".to_string()));
        assert_eq!(out.get(&3), Some(&"000".to_string()));
        assert_eq!(out.get(&4), Some(&"0000".to_string()));
    }
}
