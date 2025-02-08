
use std::char;
use std::str::FromStr;

const MOOV_IO_ACH_ADV: &str = "moov_io_ach_ADV";
const MOOV_IO_ACH_GLPRENOTEDEBIT: u8 = 48;
const MOOV_IO_ACH_GLZERODOLLARREMITTANCEDEBIT: u8 = 49;

#[derive(Debug)]
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
                let rdfi = entry.rdfi_identification.trim_start_matches(char::is_whitespace);
                if let Ok(rdfi_int) = u64::from_str(&rdfi[..8]) {
                    hash += rdfi_int as i32;
                }
            }
        } else {
            for entry in &self.adv_entries {
                let rdfi = entry.rdfi_identification.trim_start_matches(char::is_whitespace);
                if let Ok(rdfi_int) = u64::from_str(&rdfi[..8]) {
                    hash += rdfi_int as i32;
                }
            }
        }

        self.least_significant_digits(hash, 10)
    }

    fn is_adv(&self) -> bool {
        self.header.standard_entry_class_code == MOOV_IO_ACH_ADV
    }

    fn least_significant_digits(&self, v: i32, max_digits: u32) -> i32 {
        v % 10_i32.pow(max_digits as u32)
    }
}

#[derive(Debug)]
struct MoovIoAchBatchHeader {
    standard_entry_class_code: String,
}

#[derive(Debug)]
struct MoovIoAchEntryDetail {
    rdfi_identification: String,
}

#[derive(Debug)]
struct MoovIoAchADVEntryDetail {
    rdfi_identification: String,
}

fn moov_io_ach_aba8(rtn: &str) -> String {
    let n = rtn.chars().count();
    match n {
        n if n > 10 => String::new(),
        10 => {
            let first_char = rtn.chars().next().unwrap();
            if first_char == '0' || first_char == '1' {
                rtn[1..9].to_string()
            } else {
                String::new()
            }
        }
        n if n != 8 && n != 9 => String::new(),
        _ => rtn[..8].to_string(),
    }
}
