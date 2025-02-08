
extern crate lazy_static;

use std::collections::HashMap;
use std::str;
use lazy_static::lazy_static;

lazy_static! {
    static ref MOOV_IO_ACH_SPACE_ZEROS: HashMap<usize, Box<String>> = moov_io_ach_populate_map(94, " ");
    static ref MOOV_IO_ACH_STRING_ZEROS: HashMap<usize, Box<String>> = moov_io_ach_populate_map(94, "0");
}

struct MoovIoAchBatchHeader {
    company_entry_description: String,
    effective_entry_date: String,
}

impl MoovIoAchBatchHeader {
    fn effective_entry_date_field(&self) -> String {
        if self.company_entry_description == "AUTOENROLL" {
            return moov_io_ach_converters_alpha_field("", 6);
        }
        moov_io_ach_converters_string_field(&self.effective_entry_date, 6)
    }
}

fn moov_io_ach_converters_alpha_field(s: &str, max: u32) -> String {
    let ln = s.chars().count() as u32;
    if ln > max {
        return s[..max as usize].to_string();
    }

    let m = (max - ln) as usize;
    if let Some(pad) = MOOV_IO_ACH_SPACE_ZEROS.get(&m) {
        return s.to_string() + &*pad;
    }

    s.to_string() + &" ".repeat(m)
}

fn moov_io_ach_converters_string_field(s: &str, max: u32) -> String {
    let ln = s.chars().count() as u32;
    if ln > max {
        return s[..max as usize].to_string();
    }

    let m = (max - ln) as usize;
    if let Some(pad) = MOOV_IO_ACH_STRING_ZEROS.get(&m) {
        return pad.to_string() + s;
    }

    "0".repeat(m) + s
}

fn moov_io_ach_populate_map(max: usize, zero: &str) -> HashMap<usize, Box<String>> {
    let mut out = HashMap::with_capacity(max);
    for i in 0..max {
        out.insert(i, Box::new(zero.repeat(i)));
    }
    out
}
