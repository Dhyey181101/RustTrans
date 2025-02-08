
extern crate lazy_static;

use std::collections::HashMap;
use lazy_static::lazy_static;

const RECORD_LENGTH: usize = 94;
const ENTRY_ADDENDA_POS: &str = "7";

lazy_static! {
    static ref MOOV_IO_ACH_SPACE_ZEROS: HashMap<usize, String> = populate_map(94, " ");
    static ref MOOV_IO_ACH_STRING_ZEROS: HashMap<usize, String> = populate_map(94, "0");
}

fn populate_map(max: usize, zero: &str) -> HashMap<usize, String> {
    let mut out = HashMap::with_capacity(max);
    for i in 0..max {
        out.insert(i, zero.repeat(i));
    }
    out
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
            return format!("{}{}", s, pad);
        }
        // slow path
        format!("{}{}", s, " ".repeat(m))
    }

    fn numeric_field(&self, n: i64, max: u32) -> String {
        let s = n.to_string();
        let l = s.len() as u32;
        if l > max {
            return s.chars().rev().take(max as usize).collect::<String>().chars().rev().collect();
        } else {
            let m = (max - l) as usize;
            if let Some(pad) = MOOV_IO_ACH_STRING_ZEROS.get(&m) {
                return format!("{}{}", pad, s);
            }
            // slow path
            format!("{}{}", "0".repeat(m), s)
        }
    }
}

#[derive(Default)]
struct MoovIoAchAddenda05 {
    type_code: String,
    payment_related_information: String,
    sequence_number: i64,
    entry_detail_sequence_number: i64,
    converters: Box<MoovIoAchConverters>,
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
        self.converters.alpha_field(&self.payment_related_information, 80)
    }

    fn sequence_number_field(&self) -> String {
        self.converters.numeric_field(self.sequence_number, 4)
    }

    fn entry_detail_sequence_number_field(&self) -> String {
        self.converters.numeric_field(self.entry_detail_sequence_number, 7)
    }
}
