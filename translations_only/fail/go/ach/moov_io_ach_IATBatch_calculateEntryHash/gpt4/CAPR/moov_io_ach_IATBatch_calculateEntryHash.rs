
use std::str::FromStr;

const MOOV_IO_ACH_GLPRENOTE_DEBIT: u8 = 48;
const MOOV_IO_ACH_GL_ZERO_DOLLAR_REMITTANCE_DEBIT: u8 = 49;

struct MoovIoAchIATBatch {
    entries: Vec<Box<MoovIoAchIATEntryDetail>>,
    converters: MoovIoAchConverters,
}

impl MoovIoAchIATBatch {
    fn calculate_entry_hash(&self) -> i32 {
        let mut hash = 0;
        for entry in &self.entries {
            let entry_rdfi = entry.rdfi_identification.parse::<i32>().unwrap_or(0);
            hash += entry_rdfi;
        }
        self.converters.least_significant_digits(hash, 10)
    }
}

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

struct MoovIoAchConverters;

impl MoovIoAchConverters {
    fn least_significant_digits(&self, v: i32, max_digits: u32) -> i32 {
        v % 10i32.pow(max_digits)
    }
}

struct MoovIoAchIATEntryDetail {
    rdfi_identification: String,
    converters: MoovIoAchConverters,
}
