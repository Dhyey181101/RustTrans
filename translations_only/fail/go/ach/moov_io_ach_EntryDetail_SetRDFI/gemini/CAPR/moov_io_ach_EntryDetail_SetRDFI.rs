
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug)]
pub struct MoovIoAchEntryDetail {
    pub r_d_f_i_identification: String,
    pub check_digit: String,
}

impl MoovIoAchEntryDetail {
    pub fn set_r_d_f_i(&mut self, r_d_f_i: &str) -> &mut Self {
        let s = self.string_field(r_d_f_i, 9);
        self.r_d_f_i_identification = self.parse_string_field(&s[..8]);
        self.check_digit = self.parse_string_field(&s[8..9]);
        self
    }

    fn string_field(&self, s: &str, max: usize) -> String {
        let ln = s.chars().count();
        if ln > max {
            return s[..max].to_string();
        }

        // Pad with preallocated string
        let m = max - ln;
        let pad = moov_io_ach_string_zeros.get(&m).unwrap();
        pad.to_string() + s
    }

    fn parse_string_field(&self, r: &str) -> String {
        r.trim().to_string()
    }
}

lazy_static::lazy_static! {
    static ref moov_io_ach_string_zeros: HashMap<usize, String> =
        moov_io_ach_populate_map(94, "0");
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
    fn test_set_r_d_f_i() {
        let mut ed = MoovIoAchEntryDetail {
            r_d_f_i_identification: "".to_string(),
            check_digit: "".to_string(),
        };
        ed.set_r_d_f_i("123456789");
        assert_eq!(ed.r_d_f_i_identification, "12345678");
        assert_eq!(ed.check_digit, "9");
    }

    #[test]
    fn test_string_field() {
        let ed = MoovIoAchEntryDetail {};
        assert_eq!(ed.string_field("12345", 5), "12345");
        assert_eq!(ed.string_field("1234567890", 5), "12345");
        assert_eq!(ed.string_field("12345", 10), "0000012345");
    }

    #[test]
    fn test_parse_string_field() {
        let ed = MoovIoAchEntryDetail {};
        assert_eq!(ed.parse_string_field(" 12345 "), "12345");
    }

    #[test]
    fn test_moov_io_ach_populate_map() {
        let out = moov_io_ach_populate_map(10, "0");
        assert_eq!(out.get(&0), Some(&"".to_string()));
        assert_eq!(out.get(&1), Some(&"0".to_string()));
        assert_eq!(out.get(&9), Some(&"000000000".to_string()));
    }
}
