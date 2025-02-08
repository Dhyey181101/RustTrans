
use std::collections::HashMap;
use std::fmt;

pub struct MoovIoAchFileControl {
    pub block_count: i32,
}

pub struct MoovIoAchConverters {}

impl MoovIoAchFileControl {
    pub fn block_count_field(&self) -> String {
        MoovIoAchConverters {}.numeric_field(self.block_count, 6)
    }
}

impl MoovIoAchConverters {
    pub fn numeric_field(&self, n: i32, max: u32) -> String {
        let s = n.to_string();
        if s.len() as u32 > max {
            return s[(s.len() - max as usize)..].to_string();
        } else {
            let m = max - s.len() as u32;
            let pad = moov_io_ach_string_zeros(m);
            return format!("{}{}", pad, s);
        }
    }
}

fn moov_io_ach_string_zeros(m: u32) -> String {
    let mut out = String::new();
    for _ in 0..m {
        out.push('0');
    }
    out
}

fn moov_io_ach_populate_map(max: i32, zero: &str) -> HashMap<i32, String> {
    let mut out = HashMap::new();
    for i in 0..max {
        out.insert(i, zero.repeat(i as usize));
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_numeric_field() {
        assert_eq!(MoovIoAchConverters {}.numeric_field(12345, 6), "012345");
        assert_eq!(MoovIoAchConverters {}.numeric_field(123456789, 6), "345678");
    }

    #[test]
    fn test_block_count_field() {
        let fc = MoovIoAchFileControl { block_count: 12345 };
        assert_eq!(fc.block_count_field(), "012345");
    }
}
