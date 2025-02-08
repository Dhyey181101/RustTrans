

use std::collections::HashMap;
use std::str;
use once_cell::sync::Lazy;

static MOOV_IO_ACH_SPACE_ZEROS: Lazy<HashMap<usize, String>> = Lazy::new(|| moov_io_ach_populate_map(94, " "));
static MOOV_IO_ACH_STRING_ZEROS: Lazy<HashMap<usize, String>> = Lazy::new(|| moov_io_ach_populate_map(94, "0"));

const MOOV_IO_ACH_RECORD_LENGTH: usize = 94;
const MOOV_IO_ACH_ENTRY_ADDENDA_POS: &str = "7";

impl moov_io_ach_Addenda12 {
    fn string(&self) -> String {
        if self.type_code.is_empty() && self.originator_city_state_province.is_empty() && self.originator_country_postal_code.is_empty() && self.entry_detail_sequence_number == 0 {
            return String::new();
        }

        let mut buf = String::with_capacity(94);
        buf.push_str(MOOV_IO_ACH_ENTRY_ADDENDA_POS);
        buf.push_str(&self.type_code);
        buf.push_str(&self.originator_city_state_province_field());
        buf.push_str(&self.originator_country_postal_code_field());
        buf.push_str("              ");
        buf.push_str(&self.entry_detail_sequence_number_field());
        buf
    }

    fn originator_city_state_province_field(&self) -> String {
        self.alpha_field(&self.originator_city_state_province, 35)
    }

    fn originator_country_postal_code_field(&self) -> String {
        self.alpha_field(&self.originator_country_postal_code, 35)
    }

    fn entry_detail_sequence_number_field(&self) -> String {
        self.numeric_field(self.entry_detail_sequence_number, 7)
    }

    fn alpha_field(&self, s: &str, max: usize) -> String {
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

    fn numeric_field(&self, n: i32, max: usize) -> String {
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
}

fn moov_io_ach_populate_map(max: usize, zero: &str) -> HashMap<usize, String> {
    let mut out = HashMap::with_capacity(max);
    for i in 0..max {
        out.insert(i, zero.repeat(i));
    }
    out
}

struct moov_io_ach_Addenda12 {
    type_code: String,
    originator_city_state_province: String,
    originator_country_postal_code: String,
    entry_detail_sequence_number: i32,
}

