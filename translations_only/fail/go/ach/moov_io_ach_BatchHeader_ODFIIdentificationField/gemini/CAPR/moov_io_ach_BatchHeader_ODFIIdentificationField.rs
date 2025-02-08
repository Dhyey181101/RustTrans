
use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone)]
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
        let mut parts = s.split(',');
        let odfi_identification = parts.next().unwrap().to_string();
        let batch_number = parts.next().unwrap().to_string();

        Ok(MoovIoAchBatchHeader {
            odfi_identification,
            batch_number,
        })
    }
}

impl fmt::Display for MoovIoAchBatchHeader {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{},{}\n", self.odfi_identification, self.batch_number)
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
        let pad = "0".repeat(m);
        pad + s
    }

    pub fn populate_map(&self, max: usize, zero: &str) -> HashMap<usize, String> {
        let mut out = HashMap::with_capacity(max);
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
        assert_eq!(converters.string_field("12345", 5), "12345");
        assert_eq!(converters.string_field("1234567890", 5), "12345");
    }

    #[test]
    fn test_populate_map() {
        let converters = MoovIoAchConverters {};
        let map = converters.populate_map(10, "0");
        assert_eq!(map.get(&0), Some(&""));
        assert_eq!(map.get(&9), Some(&"000000000"));
    }

    #[test]
    fn test_from_str() {
        let header = MoovIoAchBatchHeader::from_str("12345678,98765432").unwrap();
        assert_eq!(header.odfi_identification, "12345678");
        assert_eq!(header.batch_number, "98765432");
    }

    #[test]
    fn test_display() {
        let header = MoovIoAchBatchHeader {
            odfi_identification: "12345678".to_string(),
            batch_number: "98765432".to_string(),
        };
        assert_eq!(header.to_string(), "12345678,98765432\n");
    }
}
