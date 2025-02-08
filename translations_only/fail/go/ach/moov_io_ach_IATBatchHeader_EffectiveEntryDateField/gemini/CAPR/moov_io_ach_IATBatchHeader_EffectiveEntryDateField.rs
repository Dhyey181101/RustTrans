
use std::collections::HashMap;
use std::fmt::Display;
use std::ops::Deref;
use std::str::FromStr;

#[derive(Debug)]
pub struct MoovIoAchIatBatchHeader {
    pub effective_entry_date: String,
    pub company_entry_description: String,
}

impl MoovIoAchIatBatchHeader {
    pub fn effective_entry_date_field(&self) -> String {
        self.string_field(&self.effective_entry_date, 6)
    }

    fn string_field(&self, s: &str, max: usize) -> String {
        let ln = s.chars().count();
        if ln > max {
            s[..max].to_string()
        } else {
            let m = max - ln;
            let pad = moov_io_ach_string_zeros(m);
            format!("{}{}", pad, s)
        }
    }
}

fn moov_io_ach_string_zeros(max: usize) -> String {
    let mut out = HashMap::new();
    for i in 0..max {
        out.insert(i, "0".repeat(i));
    }
    out.get(&max).unwrap().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_effective_entry_date_field() {
        let batch_header = MoovIoAchIatBatchHeader {
            effective_entry_date: "20230308".to_string(),
            company_entry_description: "GAS BILL".to_string(),
        };
        assert_eq!(batch_header.effective_entry_date_field(), "230308");
    }

    #[test]
    fn test_string_field() {
        let batch_header = MoovIoAchIatBatchHeader {
            effective_entry_date: "20230308".to_string(),
            company_entry_description: "GAS BILL".to_string(),
        };
        assert_eq!(batch_header.string_field("1234567890", 6), "123456");
        assert_eq!(batch_header.string_field("12345", 6), "12345 ");
    }

    #[test]
    fn test_moov_io_ach_string_zeros() {
        assert_eq!(moov_io_ach_string_zeros(6), "000000");
        assert_eq!(moov_io_ach_string_zeros(10), "0000000000");
    }
}
