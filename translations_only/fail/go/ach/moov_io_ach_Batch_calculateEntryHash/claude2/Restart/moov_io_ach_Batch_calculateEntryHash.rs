
use std::convert::TryFrom;

struct Batch {
    header: BatchHeader,
    entries: Vec<EntryDetail>,
    adv_entries: Vec<AdvEntryDetail>,
    converters: Converters,
}

struct EntryDetail {
    rdfi_identification: String,
    converters: Converters,
}

struct AdvEntryDetail {
    rdfi_identification: String,
    converters: Converters,
}

struct BatchHeader {
    standard_entry_class_code: String,
    converters: Converters,
}

struct Converters;

impl Batch {
    fn calculate_entry_hash(&self) -> i32 {
        let mut hash = 0;
        if !self.is_adv() {
            for entry in &self.entries {
                let entry_rdfi = entry
                    .rdfi_identification
                    .parse::<i32>()
                    .unwrap_or(0);
                hash += entry_rdfi;
            }
        } else {
            for entry in &self.adv_entries {
                let entry_rdfi = entry
                    .rdfi_identification
                    .parse::<i32>()
                    .unwrap_or(0);
                hash += entry_rdfi;
            }
        }

        self.converters.least_significant_digits(hash, 10)
    }

    fn is_adv(&self) -> bool {
        self.header.standard_entry_class_code
            == "moov_io_ach_ADV"
    }

    fn get_header(&self) -> &BatchHeader {
        &self.header
    }
}

impl Converters {
    fn least_significant_digits(&self, v: i32, max_digits: u32) -> i32 {
        v % 10_i32.pow(max_digits as u32) as i32
    }
}
