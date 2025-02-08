
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;

const MAX_STRING_ZEROS: usize = 94;

lazy_static! {
    static ref STRING_ZEROS: HashMap<usize, String> = (0..MAX_STRING_ZEROS)
        .map(|i| (i, String::from_utf8(vec![b'0'; i]).unwrap()))
        .collect();
}

pub struct IATEntryDetail {
    pub trace_number: String,
    pub addenda10: String,
    pub addenda11: String,
    pub addenda12: String,
    pub addenda13: String,
    pub addenda14: String,
    pub addenda15: String,
    pub addenda16: String,
    pub addenda17: Option<String>,
    pub addenda18: Option<String>,
    pub addenda98: Option<String>,
    pub addenda99: Option<String>,
}

impl IATEntryDetail {
    pub fn set_trace_number(&mut self, odfi_identification: &str, seq: usize) {
        self.trace_number = format!(
            "{}{}",
            odfi_identification.chars().take(8).collect::<String>(),
            seq.to_string().chars().rev().take(7).collect::<String>()
        );
    }
}

impl fmt::Display for IATEntryDetail {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "IATEntryDetail {{ trace_number: {}, addenda10: {}, addenda11: {}, addenda12: {}, addenda13: {}, addenda14: {}, addenda15: {}, addenda16: {}, addenda17: {:?}, addenda18: {:?}, addenda98: {:?}, addenda99: {:?} }}",
            self.trace_number, self.addenda10, self.addenda11, self.addenda12, self.addenda13, self.addenda14, self.addenda15, self.addenda16, self.addenda17, self.addenda18, self.addenda98, self.addenda99
        )
    }
}

pub fn string_field(s: &str, max: usize) -> String {
    let len = s.chars().count();
    if len > max {
        s[..max].to_string()
    } else {
        let pad = STRING_ZEROS.get(&(max - len)).unwrap();
        format!("{}{}", pad, s)
    }
}

pub fn numeric_field(n: isize, max: usize) -> String {
    let s = n.to_string();
    let len = s.chars().count();
    if len > max {
        s[len - max..].to_string()
    } else {
        let pad = STRING_ZEROS.get(&(max - len)).unwrap();
        format!("{}{}", pad, s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_field() {
        assert_eq!(string_field("1234567890", 10), "1234567890");
        assert_eq!(string_field("1234567890", 5), "12345");
        assert_eq!(string_field("12345", 10), "0000012345");
    }

    #[test]
    fn test_numeric_field() {
        assert_eq!(numeric_field(1234567890, 10), "1234567890");
        assert_eq!(numeric_field(1234567890, 5), "7890");
        assert_eq!(numeric_field(12345, 10), "00000012345");
    }
}
