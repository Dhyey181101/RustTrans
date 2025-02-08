
use std::collections::HashMap;
use std::str::FromStr;
use std::string::ToString;

#[derive(Debug)]
pub struct MoovIoAchAddenda99Contested {
    pub contested_return_code: String,
}

impl MoovIoAchAddenda99Contested {
    pub fn contested_return_code_field(&self) -> String {
        MoovIoAchConverters {}
            .string_field(&self.contested_return_code, 3)
    }
}

pub struct MoovIoAchConverters {}

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
        assert_eq!(c.string_field("12345", 3), "123");
        assert_eq!(c.string_field("12345", 5), "12345");
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
