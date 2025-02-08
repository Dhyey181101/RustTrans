
use std::collections::HashMap;

const MOOV_IO_ACH_SPACE_ZEROS: [&str; 94] = [""; 94];
const MOOV_IO_ACH_STRING_ZEROS: [&str; 94] = [""; 94];

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
            MOOV_IO_ACH_SPACE_ZEROS[m].to_string() + s
        }
    }

    fn string_field(&self, s: &str, max: usize) -> String {
        let ln = s.chars().count();
        if ln > max {
            s[..max].to_string()
        } else {
            let m = max - ln;
            MOOV_IO_ACH_STRING_ZEROS[m].to_string() + s
        }
    }
}
