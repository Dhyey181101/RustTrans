
use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;

#[derive(Debug)]
pub struct MoovIoAchFileControl {
    pub entry_addenda_count: i32,
}

impl MoovIoAchFileControl {
    pub fn entry_addenda_count_field(&self) -> String {
        MoovIoAchConverters {}.numeric_field(self.entry_addenda_count, 8)
    }
}

pub struct MoovIoAchConverters {}

impl MoovIoAchConverters {
    pub fn numeric_field(&self, n: i32, max: u32) -> String {
        let s = n.to_string();
        if s.len() as u32 > max {
            return s[(s.len() - max as usize)..].to_string();
        } else {
            let m = max - s.len() as u32;
            let pad = moov_io_ach_string_zeros(m as usize);
            return format!("{}{}", pad, s);
        }
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
        out.insert(i, moov_io_ach_string_zeros(i));
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_numeric_field() {
        assert_eq!(MoovIoAchConverters {}.numeric_field(123456789, 8), "12345678");
        assert_eq!(MoovIoAchConverters {}.numeric_field(12345678, 10), "0012345678");
    }

    #[test]
    fn test_moov_io_ach_populate_map() {
        let out = moov_io_ach_populate_map(10, "0");
        assert_eq!(out[&0], "");
        assert_eq!(out[&1], "0");
        assert_eq!(out[&9], "000000000");
    }
}
