

use std::collections::HashMap;
use std::str;

static MOOV_IO_ACH_SPACE_ZEROS: Lazy<HashMap<usize, String>> = Lazy::new(|| moov_io_ach_populate_map(94, " "));
static MOOV_IO_ACH_STRING_ZEROS: Lazy<HashMap<usize, String>> = Lazy::new(|| moov_io_ach_populate_map(94, "0"));

const MOOV_IO_ACH_RECORD_LENGTH: usize = 94;
const MOOV_IO_ACH_ENTRY_ADDENDA_POS: &str = "7";

impl moov_io_ach_Addenda16 {
    fn string(&self) -> String {
        let mut buf = String::with_capacity(94);
        buf.push_str(MOOV_IO_ACH_ENTRY_ADDENDA_POS);
        buf.push_str(&self.type_code);
        buf.push_str(&self.receiver_city_state_province_field());
        buf.push_str(&self.receiver_country_postal_code_field());
        buf.push_str("              ");
        buf.push_str(&self.entry_detail_sequence_number_field());
        buf
    }

    fn receiver_city_state_province_field(&self) -> String {
        moov_io_ach_alpha_field(&self.receiver_city_state_province, 35)
    }

    fn receiver_country_postal_code_field(&self) -> String {
        moov_io_ach_alpha_field(&self.receiver_country_postal_code, 35)
    }

    fn entry_detail_sequence_number_field(&self) -> String {
        moov_io_ach_numeric_field(self.entry_detail_sequence_number, 7)
    }
}

fn moov_io_ach_alpha_field(s: &str, max: usize) -> String {
    let ln = s.chars().count();
    if ln > max {
        return s.chars().take(max).collect();
    }

    let m = max - ln;
    if let Some(pad) = MOOV_IO_ACH_SPACE_ZEROS.get(&m) {
        return format!("{}{}", s, pad);
    }

    format!("{}{}", s, " ".repeat(m))
}

fn moov_io_ach_numeric_field(n: i32, max: usize) -> String {
    let s = n.to_string();
    if s.len() > max {
        let mut chars: Vec<char> = s.chars().rev().collect();
        chars.truncate(max);
        return chars.into_iter().rev().collect();
    }

    let m = max - s.len();
    if let Some(pad) = MOOV_IO_ACH_STRING_ZEROS.get(&m) {
        return format!("{}{}", pad, s);
    }

    format!("{}{}", "0".repeat(m), s)
}

fn moov_io_ach_populate_map(max: usize, zero: &str) -> HashMap<usize, String> {
    let mut out = HashMap::with_capacity(max);
    for i in 0..max {
        out.insert(i, zero.repeat(i));
    }
    out
}

struct moov_io_ach_Addenda16 {
    type_code: String,
    receiver_city_state_province: String,
    receiver_country_postal_code: String,
    entry_detail_sequence_number: i32,
}

use once_cell::sync::Lazy;


