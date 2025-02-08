
use std::convert::TryInto;

#[derive(Debug)]
struct MoovIoAchIatBatch {
    entries: Vec<MoovIoAchIatEntryDetail>,
}

impl MoovIoAchIatBatch {
    fn calculate_entry_hash(&self) -> i32 {
        let mut hash = 0;
        for entry in &self.entries {
            let entry_rdfi = entry.rdfi_identification.parse::<i32>().unwrap();
            hash += entry_rdfi;
        }

        hash % 10_i32.pow(10)
    }
}

#[derive(Debug)]
struct MoovIoAchIatEntryDetail {
    rdfi_identification: String,
    amount: i32,
    df_iaccount_number: String,
    trace_number: String,
}

fn moov_io_ach_aba8(rtn: &str) -> Option<String> {
    let n = rtn.chars().count();
    match n {
        10 => {
            if rtn.starts_with('0') || rtn.starts_with('1') {
                Some(rtn[1..9].to_string())
            } else {
                None
            }
        }
        8 | 9 => Some(rtn[..8].to_string()),
        _ => None,
    }
}

fn least_significant_digits(v: i32, max_digits: u32) -> i32 {
    v % 10_i32.pow(max_digits)
}
