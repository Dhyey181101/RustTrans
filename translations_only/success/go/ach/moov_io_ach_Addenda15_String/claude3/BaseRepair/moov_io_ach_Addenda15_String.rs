
extern crate lazy_static;

use std::collections::HashMap;
use std::str;
use lazy_static::lazy_static;

const RECORD_LENGTH: usize = 94;
const ENTRY_ADDENDA_POS: &str = "7";

lazy_static! {
    static ref MOOV_IO_ACH_SPACE_ZEROS: HashMap<usize, Box<String>> = populate_map(94, " ");
    static ref MOOV_IO_ACH_STRING_ZEROS: HashMap<usize, Box<String>> = populate_map(94, "0");
}

struct MoovIoAchAddenda15 {
    type_code: String,
    receiver_id_number: String,
    receiver_street_address: String,
    entry_detail_sequence_number: i32,
}

impl MoovIoAchAddenda15 {
    fn string(&self) -> String {
        let mut buf = String::with_capacity(RECORD_LENGTH);
        buf.push_str(ENTRY_ADDENDA_POS);
        buf.push_str(&self.type_code);
        buf.push_str(&self.receiver_id_number_field());
        buf.push_str(&self.receiver_street_address_field());
        buf.push_str("                                  ");
        buf.push_str(&self.entry_detail_sequence_number_field());
        buf
    }

    fn receiver_id_number_field(&self) -> String {
        alpha_field(&self.receiver_id_number, 15)
    }

    fn receiver_street_address_field(&self) -> String {
        alpha_field(&self.receiver_street_address, 35)
    }

    fn entry_detail_sequence_number_field(&self) -> String {
        numeric_field(self.entry_detail_sequence_number, 7)
    }
}

fn alpha_field(s: &str, max: usize) -> String {
    let len = s.chars().count();
    if len > max {
        s[..max].to_string()
    } else {
        let pad = MOOV_IO_ACH_SPACE_ZEROS.get(&(max - len)).unwrap_or(&Box::new(String::new())).to_string();
        format!("{}{}", s, pad)
    }
}

fn numeric_field(n: i32, max: usize) -> String {
    let s = n.to_string();
    let len = s.len();
    if len > max {
        s[len - max..].to_string()
    } else {
        let pad = MOOV_IO_ACH_STRING_ZEROS.get(&(max - len)).unwrap_or(&Box::new(String::new())).to_string();
        format!("{}{}", pad, s)
    }
}

fn populate_map(max: usize, zero: &str) -> HashMap<usize, Box<String>> {
    let mut out = HashMap::with_capacity(max);
    for i in 0..max {
        out.insert(i, Box::new(zero.repeat(i)));
    }
    out
}
