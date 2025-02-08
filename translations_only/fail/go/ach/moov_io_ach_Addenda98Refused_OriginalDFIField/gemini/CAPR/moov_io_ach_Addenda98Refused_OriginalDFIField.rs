
use std::collections::HashMap;
use std::ops::Deref;
use std::str::FromStr;

#[derive(Debug)]
pub struct MoovIoAchAddenda98Refused {
    pub original_dfi: String,
    pub corrected_data: String,
}

impl MoovIoAchAddenda98Refused {
    pub fn original_dfi_field(&self) -> &str {
        &self.original_dfi
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
        let binding = moov_io_ach_string_zeros();
        let pad = binding.get(&m).unwrap();
        format!("{}{}", pad, s)
    }
}

fn moov_io_ach_string_zeros() -> HashMap<u32, String> {
    let mut out = HashMap::new();
    for i in 0..94 {
        out.insert(i, "0".repeat(i as usize));
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_original_dfi_field() {
        let addenda98_refused = MoovIoAchAddenda98Refused {
            original_dfi: "12345678".to_string(),
            corrected_data: "98765432".to_string(),
        };

        assert_eq!(addenda98_refused.original_dfi_field(), "12345678");
    }

    #[test]
    fn test_string_field() {
        let converters = MoovIoAchConverters {};

        let s = "12345678";
        let max = 8;
        let expected = "12345678";

        assert_eq!(converters.string_field(s, max), expected);
    }
}
