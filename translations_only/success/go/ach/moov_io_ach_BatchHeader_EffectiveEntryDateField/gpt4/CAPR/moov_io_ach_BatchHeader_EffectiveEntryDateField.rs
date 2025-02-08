
use std::collections::HashMap;

lazy_static! {
    static ref MOOV_IO_ACH_SPACE_ZEROS: HashMap<usize, String> = moov_io_ach_populate_map(94, " ");
    static ref MOOV_IO_ACH_STRING_ZEROS: HashMap<usize, String> = moov_io_ach_populate_map(94, "0");
}

fn moov_io_ach_populate_map(max: usize, zero: &str) -> HashMap<usize, String> {
    let mut out = HashMap::with_capacity(max);
    for i in 0..max {
        out.insert(i, zero.repeat(i));
    }
    out
}

struct MoovIoAchBatchHeader {
    company_entry_description: String,
    effective_entry_date: String,
    converters: Box<MoovIoAchConverters>,
}

impl MoovIoAchBatchHeader {
    fn effective_entry_date_field(&self) -> String {
        if self.company_entry_description == "AUTOENROLL" {
            return self.converters.alpha_field("", 6);
        }
        self.converters.string_field(&self.effective_entry_date, 6)
    }
}

struct MoovIoAchConverters;

impl MoovIoAchConverters {
    fn alpha_field(&self, s: &str, max: usize) -> String {
        let ln = s.chars().count();
        if ln > max {
            return s.chars().take(max).collect();
        }

        let m = max - ln;
        if let Some(pad) = MOOV_IO_ACH_SPACE_ZEROS.get(&m) {
            return format!("{}{}", s, pad);
        }
        format!("{}{}", s, " ".repeat(max - ln))
    }

    fn string_field(&self, s: &str, max: usize) -> String {
        let ln = s.chars().count();
        if ln > max {
            return s.chars().take(max).collect();
        }

        let m = max - ln;
        if let Some(pad) = MOOV_IO_ACH_STRING_ZEROS.get(&m) {
            return format!("{}{}", pad, s);
        }
        format!("{}{}", "0".repeat(max - ln), s)
    }
}

#[macro_use]
extern crate lazy_static;
