
use std::collections::HashMap;
use std::fmt::Display;
use std::ops::Deref;
use std::str::FromStr;
use std::string::ToString;

#[derive(Debug, Clone)]
pub struct MoovIoAchAddenda99Contested {
    pub dishonored_return_trace_number: String,
}

impl MoovIoAchAddenda99Contested {
    pub fn dishonored_return_trace_number_field(&self) -> &str {
        &self.dishonored_return_trace_number
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

fn moov_io_ach_string_zeros(max: usize) -> String {
    let mut out = HashMap::new();
    for i in 0..max {
        out.insert(i, "0".repeat(i));
    }
    out[&max].clone()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_field() {
        let converters = MoovIoAchConverters {};
        let s = "1234567890";
        let max = 15;
        let expected = "123456789000000";
        assert_eq!(converters.string_field(s, max), expected);
    }

    #[test]
    fn test_dishonored_return_trace_number_field() {
        let addenda = MoovIoAchAddenda99Contested {
            dishonored_return_trace_number: "1234567890".to_string(),
        };
        assert_eq!(addenda.dishonored_return_trace_number_field(), "1234567890");
    }
}
