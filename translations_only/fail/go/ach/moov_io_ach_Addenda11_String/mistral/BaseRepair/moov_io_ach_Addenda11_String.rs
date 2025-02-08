

use std::fmt;
use std::collections::HashMap;
use lazy_static::lazy_static;
use regex::Regex;

const RECORD_LENGTH: usize = 94;
const ENTRY_ADENDA_POS: &str = "7";

fn moov_io_ach_populate_map(max: usize, zero: char) -> HashMap<usize, String> {
let mut out = HashMap::new();
for i in 0..max {
out.insert(i, "".repeat(i as usize).chars().map(|_| zero).collect());
}
out
}

#[derive(Default)]
struct MoovIoAchConverters;

impl MoovIoAchConverters {
fn alpha_field(&self, s: &str, max: usize) -> String {
let ln = s.chars().filter(|c| c.is_alphabetic()).count();
if ln > max {
s[..max].to_string()
} else {
let m = max - ln;
let pad = if let Some(pad) = MOOV_IO_ACH_SPACE_ZEROS.get(&m) {
pad.to_string()
} else {
" ".repeat(m)
};
s.to_string() + &pad
}
}

fn numeric_field(&self, n: i64, max: usize) -> String {
let s = n.to_string();
if s.len() > max as usize {
s[s.len() - max as usize..].to_string()
} else {
let m = max - s.len();
let pad = if let Some(pad) = MOOV_IO_ACH_STRING_ZEROS.get(&m) {
pad.to_string()
} else {
"0".repeat(m)
};
pad + &s
}
}
}

#[derive(Default)]
struct MoovIoAchAddenda1 {
// ID is a client defined string used as a reference to this record.
pub(crate) type_code: String,
pub(crate) originator_name: String,
pub(crate) originator_street_address: String,
pub(crate) entry_detail_sequence_number: i32,
moov_io_ach_converters: MoovIoAchConverters,
}

impl fmt::Display for MoovIoAchAddenda1 {
fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
let id_length = RECORD_LENGTH - self.originator_name.len() - self.originator_street_address.len() - 3 - self.entry_detail_sequence_number.to_string().len();
let id = &self.type_code[..id_length];
write!(f, "{}{}{:0>3}{}", id, self.originator_name, self.entry_detail_sequence_number, " ".repeat(RECORD_LENGTH - id.len() - self.originator_name.len() - self.originator_street_address.len() - 3 - self.entry_detail_sequence_number.to_string().len()))
}
}

lazy_static! {
static ref MOOV_IO_ACH_SPACE_ZEROS: HashMap<usize, &'static str> = {
let mut m = HashMap::new();
m.insert(1, " ");
m.insert(2, " ");
m.insert(3, " ");
m.insert(4, " ");
m.insert(5, " ");
m.insert(6, " ");
m.insert(7, " ");
m.insert(8, " ");
m.insert(9, " ");
m.insert(10, " ");
m
};
}

lazy_static! {
static ref MOOV_IO_ACH_STRING_ZEROS: HashMap<usize, &'static str> = {
let mut m = HashMap::new();
m.insert(1, "0");
m.insert(2, "00");
m.insert(3, "000");
m.insert(4, "0000");
m.insert(5, "00000");
m.insert(6, "000000");
m.insert(7, "0000000");
m.insert(8, "00000000");
m.insert(9, "000000000");
m
};
}

