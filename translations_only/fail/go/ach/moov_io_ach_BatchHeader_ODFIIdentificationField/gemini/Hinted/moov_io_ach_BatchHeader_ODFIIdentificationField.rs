
use std::collections::HashMap;
use std::fmt;
use std::ops::Deref;
use std::str::FromStr;
use std::string::ToString;

#[derive(Debug)]
pub struct MoovIoAchBatchHeader {
    pub odfi_identification: String,
    pub batch_number: String,
}

impl MoovIoAchBatchHeader {
    pub fn odfi_identification_field(&self) -> &str {
        &self.odfi_identification
    }
}

impl FromStr for MoovIoAchBatchHeader {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.splitn(2, '|');
        let odfi_identification = parts.next().unwrap();
        let batch_number = parts.next().unwrap();

        Ok(MoovIoAchBatchHeader {
            odfi_identification: odfi_identification.to_string(),
            batch_number: batch_number.to_string(),
        })
    }
}

impl ToString for MoovIoAchBatchHeader {
    fn to_string(&self) -> String {
        format!("{}|{}", self.odfi_identification, self.batch_number)
    }
}

#[derive(Debug)]
pub struct MoovIoAchConverters {}

impl MoovIoAchConverters {
    pub fn string_field(&self, s: &str, max: usize) -> String {
        let len = s.chars().count();
        if len > max {
            return s[..max].to_string();
        }

        let mut pad = String::new();
        for _ in 0..(max - len) {
            pad.push('0');
        }

        format!("{}{}", pad, s)
    }

    pub fn populate_map(&self, max: usize, zero: &str) -> HashMap<usize, String> {
        let mut out = HashMap::new();
        for i in 0..max {
            out.insert(i, zero.repeat(i));
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_field() {
        let converters = MoovIoAchConverters {};
        let s = "1234567890";
        let max = 10;
        let expected = "1234567890";
        let actual = converters.string_field(s, max);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_populate_map() {
        let converters = MoovIoAchConverters {};
        let max = 10;
        let zero = "0";
        let expected = vec!["", "0", "00", "000", "0000", "00000", "000000", "0000000", "00000000", "000000000"];
        let actual = converters.populate_map(max, zero).values().collect::<Vec<_>>();
        assert_eq!(expected, actual);
    }
}
