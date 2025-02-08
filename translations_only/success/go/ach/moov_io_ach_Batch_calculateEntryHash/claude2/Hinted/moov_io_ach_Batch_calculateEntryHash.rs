
use std::convert::TryInto;

struct MoovIoAchBatch {
    header: Option<MoovIoAchBatchHeader>,
    entries: Vec<MoovIoAchEntryDetail>,
    adv_entries: Vec<MoovIoAchAdvEntryDetail>,
    converters: Option<MoovIoAchConverters>,
}

struct MoovIoAchEntryDetail {
    rdfi_identification: String,
    converters: Option<MoovIoAchConverters>,
}

struct MoovIoAchAdvEntryDetail {
    rdfi_identification: String,
    converters: Option<MoovIoAchConverters>,
}

struct MoovIoAchBatchHeader {
    standard_entry_class_code: String,
    converters: Option<MoovIoAchConverters>,
}

struct MoovIoAchConverters;

impl MoovIoAchBatch {
    fn calculate_entry_hash(&self) -> i32 {
        let mut hash = 0;
        if self.header.as_ref().unwrap().standard_entry_class_code != "moov_io_ach_ADV" {
            for entry in &self.entries {
                let entry_rdfi: i32 = entry
                    .rdfi_identification
                    .parse()
                    .unwrap_or(0);
                hash += entry_rdfi;
            }
        } else {
            for entry in &self.adv_entries {
                let entry_rdfi: i32 = entry
                    .rdfi_identification
                    .parse()
                    .unwrap_or(0);
                hash += entry_rdfi;
            }
        }

        self.least_significant_digits(hash, 10)
    }

    fn is_adv(&self) -> bool {
        self.header.as_ref().unwrap().standard_entry_class_code == "moov_io_ach_ADV"
    }

    fn get_header(&self) -> &MoovIoAchBatchHeader {
        self.header.as_ref().unwrap()
    }

    fn least_significant_digits(&self, v: i32, max_digits: u32) -> i32 {
        v % 10_i32.pow(max_digits.try_into().unwrap())
    }
}

