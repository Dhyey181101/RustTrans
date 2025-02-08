
use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct MoovIoAchAddenda98Refused {
    pub trace_sequence_number: String,
}

impl MoovIoAchAddenda98Refused {
    pub fn trace_sequence_number_field(&self) -> String {
        self.trace_sequence_number.clone()
    }
}

impl FromStr for MoovIoAchAddenda98Refused {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let trace_sequence_number = s.chars().take(7).collect::<String>();
        Ok(MoovIoAchAddenda98Refused {
            trace_sequence_number,
        })
    }
}

impl fmt::Display for MoovIoAchAddenda98Refused {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.trace_sequence_number)
    }
}

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

fn moov_io_ach_string_zeros(m: usize) -> String {
    let mut out = String::with_capacity(m);
    for _ in 0..m {
        out.push('0');
    }
    out
}

fn moov_io_ach_populate_map(max: usize, zero: &str) -> HashMap<usize, String> {
    let mut out = HashMap::with_capacity(max);
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
        assert_eq!(c.string_field("1234567890", 7), "1234567");
        assert_eq!(c.string_field("1234567890", 10), "1234567890");
    }

    #[test]
    fn test_moov_io_ach_populate_map() {
        let out = moov_io_ach_populate_map(10, "0");
        assert_eq!(out.get(&0), Some(&"".to_string()));
        assert_eq!(out.get(&1), Some(&"0".to_string()));
        assert_eq!(out.get(&9), Some(&"000000000".to_string()));
    }

    #[test]
    fn test_moov_io_ach_addenda98_refused() {
        let addenda98_refused = MoovIoAchAddenda98Refused::from_str("1234567").unwrap();
        assert_eq!(addenda98_refused.trace_sequence_number, "1234567");
        assert_eq!(addenda98_refused.trace_sequence_number_field(), "1234567");
    }
}
