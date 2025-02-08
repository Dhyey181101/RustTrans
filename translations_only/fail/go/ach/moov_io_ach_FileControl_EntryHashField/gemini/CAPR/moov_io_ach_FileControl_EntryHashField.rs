
use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;

#[derive(Debug)]
pub struct MoovIoAchFileControl {
    pub entry_hash: i32,
}

impl MoovIoAchFileControl {
    pub fn entry_hash_field(&self) -> String {
        MoovIoAchConverters {}.numeric_field(self.entry_hash, 10)
    }
}

pub struct MoovIoAchConverters;

impl MoovIoAchConverters {
    pub fn numeric_field(&self, n: i32, max: u32) -> String {
        let s = n.to_string();
        if s.len() as u32 > max {
            return s[(s.len() - max as usize)..].to_string();
        } else {
            let m = (max - s.len() as u32) as usize;
            let pad = moov_io_ach_string_zeros(m);
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

fn moov_io_ach_populate_map(max: i32, zero: &str) -> HashMap<i32, String> {
    let mut out = HashMap::new();
    for i in 0..max {
        out.insert(i, moov_io_ach_string_zeros(i as usize));
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_numeric_field() {
        assert_eq!(MoovIoAchConverters {}.numeric_field(12345, 10), "000012345");
        assert_eq!(MoovIoAchConverters {}.numeric_field(1234567890, 10), "1234567890");
    }

    #[test]
    fn test_entry_hash_field() {
        let fc = MoovIoAchFileControl { entry_hash: 12345 };
        assert_eq!(fc.entry_hash_field(), "000012345");
    }
}
