
extern crate lazy_static;

use std::collections::HashMap;
use lazy_static::lazy_static;

const RECORD_LENGTH: usize = 94;
const ENTRY_ADDENDA_POS: &str = "7";

lazy_static! {
    static ref MOOV_IO_ACH_SPACE_ZEROS: HashMap<usize, Box<str>> = populate_map(94, " ");
    static ref MOOV_IO_ACH_STRING_ZEROS: HashMap<usize, Box<str>> = populate_map(94, "0");
}

#[derive(Default)]
struct MoovIoAchConverters {}

impl MoovIoAchConverters {
    fn alpha_field(&self, s: &str, max: u32) -> String {
        let ln = s.chars().count() as u32;
        if ln > max {
            return s.chars().take(max as usize).collect();
        }

        let m = (max - ln) as usize;
        if let Some(pad) = MOOV_IO_ACH_SPACE_ZEROS.get(&m) {
            return format!("{}{}", s, pad.as_ref());
        }

        format!("{}{}", s, " ".repeat(m))
    }

    fn numeric_field(&self, n: i32, max: u32) -> String {
        let s = n.to_string();
        let l = s.len() as u32;
        if l > max {
            return s.chars().rev().take(max as usize).collect::<String>().chars().rev().collect();
        } else {
            let m = (max - l) as usize;
            if let Some(pad) = MOOV_IO_ACH_STRING_ZEROS.get(&m) {
                return format!("{}{}", pad.as_ref(), s);
            }
            format!("{}{}", "0".repeat(m), s)
        }
    }
}

#[derive(Default)]
struct MoovIoAchAddenda12 {
    type_code: String,
    originator_city_state_province: String,
    originator_country_postal_code: String,
    entry_detail_sequence_number: i32,
    moov_io_ach_converters: MoovIoAchConverters,
}

impl MoovIoAchAddenda12 {
    fn string(&self) -> String {
        let mut buf = String::with_capacity(RECORD_LENGTH);
        buf.push_str(ENTRY_ADDENDA_POS);
        buf.push_str(&self.type_code);
        buf.push_str(&self.moov_io_ach_converters.alpha_field(&self.originator_city_state_province, 35));
        buf.push_str(&self.moov_io_ach_converters.alpha_field(&self.originator_country_postal_code, 35));
        buf.push_str("              ");
        buf.push_str(&self.moov_io_ach_converters.numeric_field(self.entry_detail_sequence_number, 7));
        buf
    }
}

fn populate_map(max: usize, zero: &str) -> HashMap<usize, Box<str>> {
    let mut out = HashMap::with_capacity(max);
    for i in 0..max {
        out.insert(i, Box::from(zero.repeat(i)));
    }
    out
}
