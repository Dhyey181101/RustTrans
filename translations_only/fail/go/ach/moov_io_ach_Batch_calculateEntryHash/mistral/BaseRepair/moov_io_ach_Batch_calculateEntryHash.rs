
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
    // ... other fields
    converters: MoovIoAchConverters,
}

struct MoovIoAchEntryDetail {
    rdfi_identification: String,
    // ... other fields
    converters: MoovIoAchConverters,
}

struct MoovIoAchAdvEntryDetail {
    rdfi_identification: String,
    // ... other fields
    converters: MoovIoAchConverters,
}

struct MoovIoAchBatch {
    header: Option<Box<MoovIoAchBatchHeader>>,
    entries: Vec<Box<MoovIoAchEntryDetail>>,
    adv_entries: Vec<Box<MoovIoAchAdvEntryDetail>>,
    converters: MoovIoAchConverters,
}

impl MoovIoAchBatch {
    fn calculate_entry_hash(&self) -> i32 {
        let mut hash = 0;

        if !self.is_adv() {
            for entry in &self.entries {
                let entry_rdi = self.rdi_from_string(&entry.rdfi_identification);
                hash += entry_rdi;
            }
        } else {
            for entry in &self.adv_entries {
                let entry_rdi = self.rdi_from_string(&entry.rdfi_identification);
                hash += entry_rdi;
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

    fn rdi_from_string(&self, rdif: &str) -> i32 {
        let rdif_bytes = rdif.as_bytes();
        let len = rdif_bytes.len();

        match len {
            10 => {
                if rdif_bytes[0] == b'0' || rdif_bytes[0] == b'1' {
                    let mut i = 1;
                    let mut sum = 0;
                    while i < 8 {
                        sum += (rdif_bytes[i] - b'0') as i32;
                        i += 1;
                    }
                    sum
                } else {
                    0
                }
            }
            8 | 9 => str::from_utf8(&rdif_bytes[..min(len, 8)]).unwrap().parse().unwrap_or(0),
            _ => 0,
        }
    }

    fn least_significant_digits(&self, v: i32, max_digits: u32) -> i32 {
        v % (10_i32.pow(max_digits as u32))
    }
}
