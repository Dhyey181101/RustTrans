
use std::collections::HashMap;
use std::fmt::Write;
use std::str::FromStr;
use std::string::ToString;

#[derive(Debug, Clone)]
pub struct MoovIoAchAddenda99Contested {
    pub contested_return_code: String,
}

impl MoovIoAchAddenda99Contested {
    pub fn contested_return_code(&self) -> String {
        string_field(&self.contested_return_code, 3)
    }
}

pub fn string_field(s: &str, max: usize) -> String {
    let ln = s.chars().count();
    if ln > max {
        return s[..max].to_string();
    }

    let m = max - ln;
    let pad = moov_io_ach_string_zeros(m);
    format!("{}{}", pad, s)
}

pub fn moov_io_ach_string_zeros(m: usize) -> String {
    let mut out = String::with_capacity(m);
    for _ in 0..m {
        write!(out, "0").unwrap();
    }
    out
}

pub fn moov_io_ach_populate_map(max: usize, zero: &str) -> HashMap<usize, String> {
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
        assert_eq!(string_field("12345", 3), "123");
        assert_eq!(string_field("12345", 5), "12345");
    }

    #[test]
    fn test_moov_io_ach_string_zeros() {
        assert_eq!(moov_io_ach_string_zeros(3), "000");
        assert_eq!(moov_io_ach_string_zeros(5), "00000");
    }

    #[test]
    fn test_moov_io_ach_populate_map() {
        let map = moov_io_ach_populate_map(3, "0");
        assert_eq!(map.get(&0), Some(&""));
        assert_eq!(map.get(&1), Some(&"0"));
        assert_eq!(map.get(&2), Some(&"00"));
    }
}
