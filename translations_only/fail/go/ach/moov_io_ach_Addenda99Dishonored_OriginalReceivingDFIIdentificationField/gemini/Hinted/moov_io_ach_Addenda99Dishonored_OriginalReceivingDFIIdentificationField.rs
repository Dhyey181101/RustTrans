
use std::collections::HashMap;
use std::ops::Deref;
use std::str::FromStr;

#[derive(Debug)]
pub struct MoovIoAchAddenda99Dishonored {
    pub original_receiving_dfi_identification: String,
}

impl MoovIoAchAddenda99Dishonored {
    pub fn original_receiving_dfi_identification(&self) -> &str {
        &self.original_receiving_dfi_identification
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
        assert_eq!(
            converters.string_field("1234567890", 10),
            "1234567890"
        );
        assert_eq!(
            converters.string_field("1234567890", 5),
            "12345"
        );
    }

    #[test]
    fn test_moov_io_ach_populate_map() {
        let out = moov_io_ach_populate_map(10, "0");
        assert_eq!(out[&0], "");
        assert_eq!(out[&1], "0");
        assert_eq!(out[&9], "000000000");
    }
}
