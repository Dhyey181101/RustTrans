
use std::cmp::min;
use std::convert::TryInto;
use std::str;
use std::string::String;
use std::vec::Vec;

const MOOV_IO_ACH_GLPrenoteDebit: u8 = 48;
const MOOV_IO_ACH_GLZeroDollarRemittanceDebit: u8 = 49;

struct MoovIoAchConverters;

impl MoovIoAchConverters {
    fn least_significant_digits(&self, v: i32, max_digits: u32) -> i32 {
        v % (10_i32.pow(max_digits as u32))
    }
}

struct MoovIoAchIatEntryDetail {
    rdfi_identification: String,
    // ... other fields elided ...
    moov_io_ach_converters: MoovIoAchConverters,
}

impl MoovIoAchIatEntryDetail {
    fn rdfi(&self) -> i32 {
        let rdfi = &self.rdfi_identification;
        let bytes = rdfi.as_bytes();
        let length = min(9, bytes.len());
        let mut result = 0;
        for i in 0..length {
            result = (result * 10) + (bytes[i] as i32 - b'0' as i32);
        }
        self.moov_io_ach_converters.least_significant_digits(result, 10)
    }
}

struct MoovIoAchIatBatch {
    entries: Vec<Box<MoovIoAchIatEntryDetail>>,
    moov_io_ach_converters: MoovIoAchConverters,
}

impl MoovIoAchIatBatch {
    fn calculate_entry_hash(&self) -> i32 {
        let mut hash = 0;
        for entry in &self.entries {
            hash += entry.rdfi();
        }
        self.moov_io_ach_converters.least_significant_digits(hash, 10)
    }
}

fn moov_io_ach_aba8(rtn: &str) -> String {
    let n = rtn.len();
    match n {
        10 => {
            if rtn.chars().nth(0).unwrap() == '0' || rtn.chars().nth(0).unwrap() == '1' {
                return rtn[1..9].to_string();
            }
        }
        8 | 9 => return rtn[..8].to_string(),
        _ => return String::new(),
    }
    String::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_moov_io_ach_aba8() {
        assert_eq!(moov_io_ach_aba8("1234567890"), "23456789".to_string());
        assert_eq!(moov_io_ach_aba8("12345678"), "12345678".to_string());
        assert_eq!(moov_io_ach_aba8("0123456789"), "12345678".to_string());
        assert_eq!(moov_io_ach_aba8("01234567"), "01234567".to_string());
        assert_eq!(moov_io_ach_aba8("1234567"), String::new());
        assert_eq!(moov_io_ach_aba8("123456"), String::new());
        assert_eq!(moov_io_ach_aba8("12345"), String::new());
        assert_eq!(moov_io_ach_aba8("1234"), String::new());
        assert_eq!(moov_io_ach_aba8("123"), String::new());
        assert_eq!(moov_io_ach_aba8("12"), String::new());
        assert_eq!(moov_io_ach_aba8("1"), String::new());
    }
}
