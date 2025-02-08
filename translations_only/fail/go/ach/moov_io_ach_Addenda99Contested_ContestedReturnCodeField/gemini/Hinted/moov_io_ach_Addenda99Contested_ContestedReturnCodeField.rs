
use std::collections::HashMap;
use std::fmt::Display;
use std::ops::Deref;
use std::str::FromStr;
use std::string::ToString;

#[derive(Debug, Clone)]
pub struct MoovIoAchAddenda99Contested {
    pub contested_return_code: String,
}

impl MoovIoAchAddenda99Contested {
    pub fn contested_return_code_field(&self) -> String {
        self.string_field(&self.contested_return_code, 3)
    }

    fn string_field(&self, s: &str, max: usize) -> String {
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
        let contested = MoovIoAchAddenda99Contested {
            contested_return_code: "12345".to_string(),
        };
        assert_eq!(contested.string_field("12345", 3), "123");
        assert_eq!(contested.string_field("12345", 5), "12345");
    }

    #[test]
    fn test_contested_return_code_field() {
        let contested = MoovIoAchAddenda99Contested {
            contested_return_code: "12345".to_string(),
        };
        assert_eq!(contested.contested_return_code_field(), "123");
    }
}
