
use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;
use std::string::ToString;

#[derive(Debug, Clone)]
pub struct Addenda99 {
    pub trace_number: String,
}

impl Addenda99 {
    pub fn trace_number_field(&self) -> String {
        self.trace_number.to_string()
    }
}

impl ToString for Addenda99 {
    fn to_string(&self) -> String {
        format!("Addenda99 {{ trace_number: {} }}", self.trace_number)
    }
}

impl FromStr for Addenda99 {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut addenda99 = Addenda99 {
            trace_number: String::new(),
        };

        let mut parts = s.split(',');
        if let Some(trace_number) = parts.next() {
            addenda99.trace_number = trace_number.to_string();
        }

        Ok(addenda99)
    }
}

pub fn string_field(s: &str, max: usize) -> String {
    let ln = s.chars().count();
    if ln > max {
        return s[..max].to_string();
    }

    let m = max - ln;
    let pad = "0".repeat(m);
    format!("{}{}", pad, s)
}

pub fn populate_map(max: usize, zero: &str) -> HashMap<usize, String> {
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
        assert_eq!(string_field("12345", 5), "12345");
        assert_eq!(string_field("123456", 5), "12345");
        assert_eq!(string_field("1234567", 5), "12345");
    }

    #[test]
    fn test_populate_map() {
        let map = populate_map(5, "0");
        assert_eq!(map.get(&0), Some(&""));
        assert_eq!(map.get(&1), Some(&"0"));
        assert_eq!(map.get(&2), Some(&"00"));
        assert_eq!(map.get(&3), Some(&"000"));
        assert_eq!(map.get(&4), Some(&"0000"));
    }

    #[test]
    fn test_addenda99() {
        let addenda99 = Addenda99 {
            trace_number: "12345".to_string(),
        };

        assert_eq!(addenda99.trace_number_field(), "12345");
        assert_eq!(addenda99.to_string(), "Addenda99 { trace_number: 12345 }");
    }
}
