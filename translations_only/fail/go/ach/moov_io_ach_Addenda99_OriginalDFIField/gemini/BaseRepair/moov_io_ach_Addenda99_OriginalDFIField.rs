
use std::collections::HashMap;
use std::fmt::Display;
use std::str::FromStr;

#[derive(Debug)]
pub struct Addenda99 {
    pub original_dfi: String,
    pub addenda_information: String,
    pub trace_number: String,
}

impl Addenda99 {
    pub fn original_dfi_field(&self) -> String {
        self.string_field(&self.original_dfi, 8)
    }

    fn string_field(&self, s: &str, max: usize) -> String {
        let ln = s.chars().count();
        if ln > max {
            return s[..max].to_string();
        }

        let m = max - ln;
        let pad = string_zeros(m);
        pad + s
    }
}

fn string_zeros(m: usize) -> String {
    let mut out = String::new();
    for _ in 0..m {
        out.push('0');
    }
    out
}

fn populate_map(max: usize, zero: char) -> HashMap<usize, String> {
    let mut out = HashMap::new();
    for i in 0..max {
        out.insert(i, string_zeros(i));
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_field() {
        let addenda99 = Addenda99 {
            original_dfi: "123456789".to_string(),
            addenda_information: "".to_string(),
            trace_number: "".to_string(),
        };

        assert_eq!(addenda99.string_field("123456789", 8), "12345678");
        assert_eq!(addenda99.string_field("123456789", 10), "123456789 ");
    }

    #[test]
    fn test_original_dfi_field() {
        let addenda99 = Addenda99 {
            original_dfi: "123456789".to_string(),
            addenda_information: "".to_string(),
            trace_number: "".to_string(),
        };

        assert_eq!(addenda99.original_dfi_field(), "12345678");
    }
}
