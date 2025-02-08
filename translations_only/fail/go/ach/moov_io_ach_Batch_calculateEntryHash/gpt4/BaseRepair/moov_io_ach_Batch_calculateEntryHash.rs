
use std::collections::HashMap;

const MOOV_IO_ACH_ADV: &str = "moov_io_ach_ADV";
const MOOV_IO_ACH_GLPRENOTEDBIT: i32 = 48;
const MOOV_IO_ACH_GLZERODOLLARREMITTANCEDEBIT: i32 = 49;

struct MoovIoAchBatch {
    header: Box<MoovIoAchBatchHeader>,
    entries: Vec<Box<MoovIoAchEntryDetail>>,
    adv_entries: Vec<Box<MoovIoAchADVEntryDetail>>,
}

impl MoovIoAchBatch {
    fn calculate_entry_hash(&self) -> i32 {
        let mut hash = 0;

        if !self.is_adv() {
            for entry in &self.entries {
                let entry_rdfi: i32 = entry.rdfi_identification.parse().unwrap_or(0);
                hash += entry_rdfi;
            }
        } else {
            for entry in &self.adv_entries {
                let entry_rdfi: i32 = entry.rdfi_identification.parse().unwrap_or(0);
                hash += entry_rdfi;
            }
        }

        least_significant_digits(hash, 10)
    }

    fn is_adv(&self) -> bool {
        self.header.standard_entry_class_code == MOOV_IO_ACH_ADV
    }

    fn get_header(&self) -> &MoovIoAchBatchHeader {
        &self.header
    }
}

fn least_significant_digits(v: i32, max_digits: u32) -> i32 {
    v % 10_i32.pow(max_digits)
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

struct MoovIoAchEntryDetail {
    rdfi_identification: String,
}

struct MoovIoAchADVEntryDetail {
    rdfi_identification: String,
}

struct MoovIoAchBatchHeader {
    standard_entry_class_code: String,
}
