
use std::str::FromStr;

const MOOV_IO_ACH_GLPRENOTEDEBIT: u32 = 48;
const MOOV_IO_ACH_GLZERODOLLARREMITTANCEDEBIT: u32 = 49;

struct MoovIoAchIatBatch {
    entries: Vec<MoovIoAchIatEntryDetail>,
}

impl MoovIoAchIatBatch {
    fn calculate_entry_hash(&self) -> i32 {
        let mut hash = 0;
        for entry in &self.entries {
            let entry_rdfi: i32 = i32::from_str(&moov_io_ach_aba8(&entry.rdfi_identification)).unwrap_or(0);
            hash += entry_rdfi;
        }

        let converters = MoovIoAchConverters;
        converters.least_significant_digits(hash, 10)
    }
}

fn moov_io_ach_aba8(rtn: &str) -> &str {
    let n = rtn.len();

    match n {
        n if n > 10 => "",
        10 => {
            if &rtn[0..1] == "0" || &rtn[0..1] == "1" {
                &rtn[1..9]
            } else {
                ""
            }
        }
        n if n != 8 && n != 9 => "",
        _ => &rtn[0..8],
    }
}

struct MoovIoAchConverters;

impl MoovIoAchConverters {
    fn least_significant_digits(&self, v: i32, max_digits: u32) -> i32 {
        let pow = 10f64.powi(max_digits as i32);
        v % pow as i32
    }
}

struct MoovIoAchIatEntryDetail {
    rdfi_identification: String,
}

