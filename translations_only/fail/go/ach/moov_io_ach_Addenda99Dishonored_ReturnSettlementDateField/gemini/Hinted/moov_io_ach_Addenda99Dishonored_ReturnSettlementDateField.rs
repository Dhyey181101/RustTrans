
use std::collections::HashMap;
use std::fmt::Write;

pub struct MoovIoAchAddenda99Dishonored {
    return_settlement_date: String,
}

impl MoovIoAchAddenda99Dishonored {
    pub fn return_settlement_date(&self) -> &str {
        &self.return_settlement_date
    }
}

pub struct MoovIoAchConverters {}

impl MoovIoAchConverters {
    pub fn string_field(&self, s: &str, max: u32) -> String {
        let ln = s.chars().count() as u32;
        if ln > max {
            s[..max as usize].to_string()
        } else {
            let mut pad = String::new();
            let m = max - ln;
            for _ in 0..m {
                write!(&mut pad, "0").unwrap();
            }
            pad + s
        }
    }
}

pub fn moov_io_ach_populate_map(max: usize, zero: &str) -> HashMap<usize, String> {
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
        assert_eq!(c.string_field("12345", 5), "12345");
        assert_eq!(c.string_field("12345", 6), "123450");
        assert_eq!(c.string_field("12345", 4), "1234");
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
