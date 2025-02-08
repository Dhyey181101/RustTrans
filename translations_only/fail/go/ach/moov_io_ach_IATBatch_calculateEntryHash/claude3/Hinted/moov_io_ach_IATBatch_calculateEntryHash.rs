
use std::char;

const MOOV_IO_ACH_GL_PRENOTE_DEBIT: i32 = 48;
const MOOV_IO_ACH_GL_ZERO_DOLLAR_REMITTANCE_DEBIT: i32 = 49;

fn moov_io_ach_aba8(rtn: &str) -> String {
    let n = rtn.chars().count();
    match n {
        n if n > 10 => String::new(),
        10 => {
            if rtn.starts_with('0') || rtn.starts_with('1') {
                rtn[1..9].to_string()
            } else {
                String::new()
            }
        }
        n if n != 8 && n != 9 => String::new(),
        _ => rtn[..8].to_string(),
    }
}

fn least_significant_digits(v: i32, max_digits: u32) -> i32 {
    v % 10_i32.pow(max_digits as u32)
}

struct MoovIoAchIATBatch {
    entries: Vec<Box<MoovIoAchIATEntryDetail>>,
}

impl MoovIoAchIATBatch {
    fn calculate_entry_hash(&self) -> i32 {
        let mut hash = 0;
        for entry in &self.entries {
            if let Ok(entry_rdfi) = entry.rdfi_identification.parse::<i32>() {
                hash += entry_rdfi;
            }
        }
        least_significant_digits(hash, 10)
    }
}

struct MoovIoAchIATEntryDetail {
    rdfi_identification: String,
}

struct MoovIoAchConverters {}
