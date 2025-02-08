
use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;

#[derive(Debug)]
pub struct MoovIoAchAddenda99Contested {
    pub dishonored_return_reason_code: String,
}

impl MoovIoAchAddenda99Contested {
    pub fn dishonored_return_reason_code_field(&self) -> &str {
        &self.dishonored_return_reason_code
    }
}

impl FromStr for MoovIoAchAddenda99Contested {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut dishonored_return_reason_code = String::new();
        dishonored_return_reason_code.push_str(&s[0..2]);
        Ok(MoovIoAchAddenda99Contested {
            dishonored_return_reason_code,
        })
    }
}

pub struct MoovIoAchConverters {}

impl MoovIoAchConverters {
    pub fn string_field(&self, s: &str, max: u32) -> String {
        let ln = s.chars().count() as u32;
        if ln > max {
            return s[0..max as usize].to_string();
        }

        let m = max - ln;
        let pad = moov_io_ach_string_zeros(m as usize);
        format!("{}{}", pad, s)
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
        let converters = MoovIoAchConverters {};
        assert_eq!(
            converters.string_field("12345", 5),
            "12345".to_string()
        );
        assert_eq!(
            converters.string_field("12345", 6),
            "12345 ".to_string()
        );
        assert_eq!(
            converters.string_field("12345", 7),
            "12345  ".to_string()
        );
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

    #[test]
    fn test_from_str() {
        let contested = MoovIoAchAddenda99Contested::from_str("12").unwrap();
        assert_eq!(contested.dishonored_return_reason_code, "12");
    }
}
