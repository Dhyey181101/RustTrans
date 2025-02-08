
use std::str::FromStr;

struct MoovIoAchIATBatch {
    entries: Vec<Box<MoovIoAchIATEntryDetail>>,
}

struct MoovIoAchIATEntryDetail {
    rdfi_identification: String,
}

struct MoovIoAchConverters;

impl MoovIoAchIATBatch {
    fn calculate_entry_hash(&self) -> i32 {
        let mut hash = 0;
        for entry in &self.entries {
            let entry_rdfi = MoovIoAchConverters::moov_io_ach_aba8(&entry.rdfi_identification);
            let entry_rdfi = i32::from_str(&entry_rdfi).unwrap_or(0);
            hash += entry_rdfi;
        }
        MoovIoAchConverters::least_significant_digits(hash, 10)
    }
}

impl MoovIoAchConverters {
    fn moov_io_ach_aba8(rtn: &str) -> String {
        let n = rtn.chars().count();
        match n {
            n if n > 10 => "".to_string(),
            10 => {
                if rtn.starts_with('0') || rtn.starts_with('1') {
                    rtn[1..9].to_string()
                } else {
                    "".to_string()
                }
            }
            _ if n != 8 && n != 9 => "".to_string(),
            _ => rtn[..8].to_string(),
        }
    }

    fn least_significant_digits(v: i32, max_digits: u32) -> i32 {
        v % 10_i32.pow(max_digits)
    }
}
