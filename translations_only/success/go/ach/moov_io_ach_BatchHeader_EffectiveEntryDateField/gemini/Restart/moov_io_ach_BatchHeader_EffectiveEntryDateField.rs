
use lazy_static::lazy_static;

use std::collections::HashMap;

const MAX_SIZE: usize = 94;

lazy_static! {
    static ref MOOV_IO_ACH_SPACE_ZEROS: HashMap<usize, String> =
        moov_io_ach_populate_map(MAX_SIZE, " ");
    static ref MOOV_IO_ACH_STRING_ZEROS: HashMap<usize, String> =
        moov_io_ach_populate_map(MAX_SIZE, "0");
}

pub struct MoovIoAchBatchHeader {
    pub company_entry_description: String,
    pub effective_entry_date: String,
}

impl MoovIoAchBatchHeader {
    pub fn effective_entry_date_field(&self) -> String {
        if self.company_entry_description == "AUTOENROLL" {
            self.alpha_field("", 6)
        } else {
            self.string_field(&self.effective_entry_date, 6)
        }
    }

    fn alpha_field(&self, s: &str, max: usize) -> String {
        let ln = s.chars().count();
        if ln > max {
            s[..max].to_string()
        } else {
            let m = max - ln;
            let pad = MOOV_IO_ACH_SPACE_ZEROS.get(&m).unwrap();
            format!("{}{}", s, pad)
        }
    }

    fn string_field(&self, s: &str, max: usize) -> String {
        let ln = s.chars().count();
        if ln > max {
            s[..max].to_string()
        } else {
            let m = max - ln;
            let pad = MOOV_IO_ACH_STRING_ZEROS.get(&m).unwrap();
            format!("{}{}", pad, s)
        }
    }
}

fn moov_io_ach_populate_map(max: usize, zero: &str) -> HashMap<usize, String> {
    let mut out = HashMap::new();
    for i in 0..max {
        out.insert(i, zero.repeat(i));
    }
    out
}
