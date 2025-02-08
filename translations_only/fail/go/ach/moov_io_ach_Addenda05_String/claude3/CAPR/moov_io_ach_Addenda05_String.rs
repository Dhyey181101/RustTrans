
extern crate lazy_static;

use std::collections::HashMap;
use std::str;
use lazy_static::lazy_static;

const RECORD_LENGTH: usize = 94;
const ENTRY_ADDENDA_POS: &str = "7";

lazy_static! {
    static ref MOOV_IO_ACH_SPACE_ZEROS: Box<HashMap<usize, String>> = Box::new(populate_map(94, " "));
    static ref MOOV_IO_ACH_STRING_ZEROS: Box<HashMap<usize, String>> = Box::new(populate_map(94, "0"));
}

fn populate_map(max: usize, zero: &str) -> HashMap<usize, String> {
    let mut out = HashMap::with_capacity(max);
    for i in 0..max {
        out.insert(i, zero.repeat(i));
    }
    out
}

struct MoovIoAchAddenda05 {
    type_code: String,
    payment_related_information: String,
    sequence_number: usize,
    entry_detail_sequence_number: usize,
}

impl MoovIoAchAddenda05 {
    fn string(&self) -> String {
        let mut buf = String::with_capacity(RECORD_LENGTH);
        buf.push_str(ENTRY_ADDENDA_POS);
        buf.push_str(&self.type_code);
        buf.push_str(&self.payment_related_information_field());
        buf.push_str(&self.sequence_number_field());
        buf.push_str(&self.entry_detail_sequence_number_field());
        buf
    }

    fn payment_related_information_field(&self) -> String {
        alpha_field(&self.payment_related_information, 80)
    }

    fn sequence_number_field(&self) -> String {
        numeric_field(self.sequence_number, 4)
    }

    fn entry_detail_sequence_number_field(&self) -> String {
        numeric_field(self.entry_detail_sequence_number, 7)
    }
}

fn alpha_field(s: &str, max: usize) -> String {
    let ln = s.chars().count();
    if ln > max {
        return s[..max].to_string();
    }

    let m = max - ln;
    if let Some(pad) = MOOV_IO_ACH_SPACE_ZEROS.get(&m) {
        return format!("{}{}", s, pad);
    }

    format!("{}{}", s, " ".repeat(m))
}

fn numeric_field(n: usize, max: usize) -> String {
    let s = n.to_string();
    let l = s.len();
    if l > max {
        return s[l - max..].to_string();
    }

    let m = max - l;
    if let Some(pad) = MOOV_IO_ACH_STRING_ZEROS.get(&m) {
        return format!("{}{}", pad, s);
    }

    format!("{}{}", "0".repeat(m), s)
}
