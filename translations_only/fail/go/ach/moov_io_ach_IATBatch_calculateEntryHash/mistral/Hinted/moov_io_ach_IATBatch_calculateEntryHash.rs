

use std::cmp::min;
use std::str;

const MOOV_IO_ACH_GLPRENOTEDEBIT: u8 = 48;
const MOOV_IO_ACH_GLZERODOLLARREMITTANCEDEBIT: u8 = 49;

struct MoovIoAchConverters;

struct MoovIoAchIATEntryDetail {
    rdfi_identification: String,
    // ... other fields
}

struct MoovIoAchIATBatch {
    entries: Vec<Box<MoovIoAchIATEntryDetail>>,
    // ... other fields
}

impl MoovIoAchIATBatch {
    fn calculate_entry_hash(&self) -> i32 {
        let mut hash = 0;
        for entry in &self.entries {
            let entry_rdfi = match str::from_utf8(&entry.rdfi_identification.as_bytes()) {
                Ok(rdfi) => {
                    let rdfi_len = rdfi.len();
                    if rdfi_len >= 8 && rdfi_len <= 20 {
                        rdfi[..8].to_string()
                    } else {
                        String::new()
                    }
                }
                Err(_) => String::new(),
            };
            let entry_rdfi_i32 = match entry_rdfi.parse::<i32>() {
                Ok(n) => n,
                Err(_) => 0,
            };
            hash += entry_rdfi_i32;
        }

        return self.least_significant_digits(hash, 10);
    }

    fn least_significant_digits(&self, v: i32, max_digits: u32) -> i32 {
        return v % (10_i32.pow(max_digits as u32));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_entry_hash() {
        let entries0 = vec![
            Box::new(MoovIoAchIATEntryDetail {
                rdfi_identification: "".to_string(),
                // ... other fields
            }),
            Box::new(MoovIoAchIATEntryDetail {
                rdfi_identification: "".to_string(),
                // ... other fields
            }),
            Box::new(MoovIoAchIATEntryDetail {
                rdfi_identification: "0".to_string(),
                // ... other fields
            }),
            Box::new(MoovIoAchIATEntryDetail {
                rdfi_identification: "4".to_string(),
                // ... other fields
            }),
            Box::new(MoovIoAchIATEntryDetail {
                rdfi_identification: "-".to_string(),
                // ... other fields
            }),
            // ... and 4 other elements
        ];
        let iat_batch0 = MoovIoAchIATBatch { entries: entries0 };
        assert_eq!(iat_batch0.calculate_entry_hash(), 0);

        let entries1 = vec![
            Box::new(MoovIoAchIATEntryDetail {
                rdfi_identification: "/".to_string(),
                // ... other fields
            }),
            Box::new(MoovIoAchIATEntryDetail {
                rdfi_identification: ":".to_string(),
                // ... other fields
            }),
            Box::new(MoovIoAchIATEntryDetail {
                rdfi_identification: "".to_string(),
                // ... other fields
            }),
            Box::new(MoovIoAchIATEntryDetail {
                rdfi_identification: "".to_string(),
                // ... other fields
            }),
            // ... and 1 other elements
        ];
        let iat_batch1 = MoovIoAchIATBatch {
            entries: entries1,
        };
        assert_eq!(iat_batch1.calculate_entry_hash(), 0);
    }
}

