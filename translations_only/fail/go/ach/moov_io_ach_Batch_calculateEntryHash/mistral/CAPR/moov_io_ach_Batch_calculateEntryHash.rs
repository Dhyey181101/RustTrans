
use std::cmp::min;
use std::convert::TryInto;
use std::str;

const MOOV_IO_ACH_ADV: &str = "moov_io_ach_ADV";
const MOOV_IO_ACH_GLPrenoteDebit: i32 = 48;
const MOOV_IO_ACH_GLZeroDollarRemittanceDebit: i32 = 49;

struct MoovIoAchConverters;

impl MoovIoAchConverters {
    fn least_significant_digits(&self, v: i32, max_digits: u32) -> i32 {
        v % (10_i32.pow(max_digits as u32))
    }
}

struct MoovIoAchBatchHeader {
    standard_entry_class_code: String,
}

struct MoovIoAchEntryDetail {
    rdfi_identification: String,
}

struct MoovIoAchADVEntryDetail {
    rdfi_identification: String,
}

struct MoovIoAchBatch {
    header: Option<Box<MoovIoAchBatchHeader>>,
    entries: Vec<Box<MoovIoAchEntryDetail>>,
    adv_entries: Vec<Box<MoovIoAchADVEntryDetail>>,
    moov_io_ach_converters: MoovIoAchConverters,
}

impl MoovIoAchBatch {
    fn calculate_entry_hash(&self) -> i32 {
        let mut hash = 0;

        if !self.is_adv() {
            for entry in &self.entries {
                let entry_rdfi = str::parse::<i32>(&moov_io_ach_aba8(&entry.rdfi_identification)).unwrap_or(0);
                hash += entry_rdfi;
            }
        } else {
            for entry in &self.adv_entries {
                let entry_rdfi = str::parse::<i32>(&moov_io_ach_aba8(&entry.rdfi_identification)).unwrap_or(0);
                hash += entry_rdfi;
            }
        }

        self.least_significant_digits(hash, 10)
    }

    fn is_adv(&self) -> bool {
        match &self.header {
            Some(header) => header.standard_entry_class_code == MOOV_IO_ACH_ADV,
            None => false,
        }
    }

    fn least_significant_digits(&self, v: i32, max_digits: u32) -> i32 {
        self.moov_io_ach_converters.least_significant_digits(v, max_digits)
    }
}

fn moov_io_ach_aba8(rtn: &str) -> String {
    let n = rtn.len();
    match n {
        10 => {
            if rtn.chars().nth(0).unwrap() == '0' || rtn.chars().nth(0).unwrap() == '1' {
                rtn[1..9].to_string()
            } else {
                String::new()
            }
        }
        8 | 9 => rtn[..8].to_string(),
        _ => String::new(),
    }
}

fn main() {}
