

use std::fmt;
use std::collections::HashMap;

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
let pad = if let Some(pad) = moov_io_ach_get_pad_string(" ", m) {
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
let pad = if let Some(pad) = moov_io_ach_get_pad_string("0", m) {
pad.to_string()
} else {
"0".repeat(m)
};
pad + &s
}
}
}

fn moov_io_ach_get_pad_string(c: &str, n: usize) -> Option<&'static str> {
match n {
1 => Some(" "),
2 => Some("  "),
3 => Some("   "),
4 => Some("    "),
5 => Some("     "),
6 => Some("      "),
7 => Some("       "),
8 => Some("        "),
9 => Some("         "),
10 => Some("          "),
11 => Some("           "),
12 => Some("            "),
13 => Some("             "),
14 => Some("              "),
15 => Some("               "),
_ => None,
}
}

#[derive(Default)]
struct MoovIoAchAddenda1 {
// ID is a client defined string used as a reference to this record.
pub(crate) type_code: String,
pub(crate) originator_name: String,
pub(crate) originator_street_address: String,
pub(crate) entry_detail_sequence_number: i64,
moov_io_ach_converters: MoovIoAchConverters,
}

impl fmt::Display for MoovIoAchAddenda1 {
fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
write!(f, "{}{}{}{:0>8}{}",
self.type_code,
self.originator_name,
self.originator_street_address,
self.entry_detail_sequence_number,
ENTRY_ADENDA_POS
)
}
}

