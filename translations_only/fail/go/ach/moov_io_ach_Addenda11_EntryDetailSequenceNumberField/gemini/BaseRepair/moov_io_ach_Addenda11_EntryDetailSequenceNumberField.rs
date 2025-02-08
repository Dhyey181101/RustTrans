
use std::collections::HashMap;
use std::fmt::Display;

#[derive(Debug)]
pub struct Addenda11 {
    pub entry_detail_sequence_number: i32,
}

impl Addenda11 {
    pub fn entry_detail_sequence_number_field(&self) -> String {
        self.numeric_field(self.entry_detail_sequence_number, 7)
    }

    fn numeric_field(&self, n: i32, max: u32) -> String {
        let s = n.to_string();
        if s.len() as u32 > max {
            return s[(s.len() - max as usize)..].to_string();
        } else {
            let m = max as usize - s.len();
            let pad = string_zeros(m);
            return pad + &s;
        }
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
    fn test_entry_detail_sequence_number_field() {
        let addenda11 = Addenda11 {
            entry_detail_sequence_number: 1234567,
        };
        assert_eq!(addenda11.entry_detail_sequence_number_field(), "1234567");
    }

    #[test]
    fn test_numeric_field() {
        let addenda11 = Addenda11 {
            entry_detail_sequence_number: 1234567,
        };
        assert_eq!(addenda11.numeric_field(1234567, 7), "1234567");
        assert_eq!(addenda11.numeric_field(12345678, 7), "2345678");
        assert_eq!(addenda11.numeric_field(123456789, 7), "3456789");
    }

    #[test]
    fn test_string_zeros() {
        assert_eq!(string_zeros(0), "");
        assert_eq!(string_zeros(1), "0");
        assert_eq!(string_zeros(2), "00");
        assert_eq!(string_zeros(3), "000");
    }

    #[test]
    fn test_populate_map() {
        let map = populate_map(10, '0');
        assert_eq!(map.get(&0), Some(&"".to_string()));
        assert_eq!(map.get(&1), Some(&"0".to_string()));
        assert_eq!(map.get(&2), Some(&"00".to_string()));
        assert_eq!(map.get(&3), Some(&"000".to_string()));
        assert_eq!(map.get(&9), Some(&"000000000".to_string()));
    }
}
