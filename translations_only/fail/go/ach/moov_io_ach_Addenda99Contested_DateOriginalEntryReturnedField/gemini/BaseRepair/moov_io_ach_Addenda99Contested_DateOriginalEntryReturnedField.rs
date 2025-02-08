
use std::collections::HashMap;
use std::fmt;
use std::ops::Deref;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct MoovIoAchAddenda99Contested {
    pub date_original_entry_returned: String,
}

impl MoovIoAchAddenda99Contested {
    pub fn date_original_entry_returned_field(&self) -> &str {
        &self.date_original_entry_returned
    }
}

#[derive(Debug, Clone)]
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
        let max = 6;
        let expected = "123456";
        let actual = converters.string_field(s, max);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_moov_io_ach_populate_map() {
        let max = 10;
        let zero = "0";
        let expected = vec![
            "".to_string(),
            "0".to_string(),
            "00".to_string(),
            "000".to_string(),
            "0000".to_string(),
            "00000".to_string(),
            "000000".to_string(),
            "0000000".to_string(),
            "00000000".to_string(),
            "000000000".to_string(),
        ];
        let actual = moov_io_ach_populate_map(max, zero);
        for (i, v) in expected.iter().enumerate() {
            assert_eq!(v, actual.get(&i).unwrap());
        }
    }
}
