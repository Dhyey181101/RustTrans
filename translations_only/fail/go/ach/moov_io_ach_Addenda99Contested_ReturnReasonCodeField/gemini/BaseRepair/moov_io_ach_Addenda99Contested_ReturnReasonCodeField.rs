
use std::collections::HashMap;
use std::fmt::Display;
use std::str::FromStr;

#[derive(Debug)]
pub struct MoovIoAchAddenda99Contested {
    pub return_reason_code: String,
}

impl MoovIoAchAddenda99Contested {
    pub fn return_reason_code_field(&self) -> &str {
        &self.return_reason_code
    }
}

#[derive(Debug)]
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
        let converters = MoovIoAchConverters {};
        let s = "1234567890";
        let max = 5;
        let expected = "12345";
        let actual = converters.string_field(s, max);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_moov_io_ach_populate_map() {
        let max = 5;
        let zero = "0";
        let expected = HashMap::from([
            (0, "".to_string()),
            (1, "0".to_string()),
            (2, "00".to_string()),
            (3, "000".to_string()),
            (4, "0000".to_string()),
        ]);
        let actual = moov_io_ach_populate_map(max, zero);
        assert_eq!(expected, actual);
    }
}
