

use std::collections::HashMap;
use std::str;
use std::ops::Deref;
use once_cell::sync::Lazy;

static MOOV_IO_ACH_SPACE_ZEROS: Lazy<HashMap<usize, String>> = Lazy::new(|| {
    let mut out = HashMap::with_capacity(94);
    for i in 0..94 {
        out.insert(i, " ".repeat(i));
    }
    out
});

static MOOV_IO_ACH_STRING_ZEROS: Lazy<HashMap<usize, String>> = Lazy::new(|| {
    let mut out = HashMap::with_capacity(94);
    for i in 0..94 {
        out.insert(i, "0".repeat(i));
    }
    out
});

const MOOV_IO_ACH_RECORD_LENGTH: usize = 94;
const MOOV_IO_ACH_ENTRY_ADDENDA_POS: &str = "7";

impl MoovIoAchAddenda05 {
    fn string(&self) -> String {
        let mut buf = String::with_capacity(94);
        buf.push_str(MOOV_IO_ACH_ENTRY_ADDENDA_POS);
        buf.push_str(&self.type_code);
        buf.push_str(&self.payment_related_information_field());
        buf.push_str(&self.sequence_number_field());
        buf.push_str(&self.entry_detail_sequence_number_field());
        buf
    }

    fn payment_related_information_field(&self) -> String {
        self.alpha_field(&self.payment_related_information, 80)
    }

    fn sequence_number_field(&self) -> String {
        self.numeric_field(self.sequence_number, 4)
    }

    fn entry_detail_sequence_number_field(&self) -> String {
        self.numeric_field(self.entry_detail_sequence_number, 7)
    }
}

impl MoovIoAchAddenda05 {
    fn alpha_field(&self, s: &str, max: u32) -> String {
        let ln = s.chars().count() as u32;
        if ln > max {
            return s[..max as usize].to_string();
        }

        let m = (max - ln) as usize;
        if let Some(pad) = MOOV_IO_ACH_SPACE_ZEROS.deref().get(&m) {
            return format!("{}{}", s, pad);
        }
        // slow path
        format!("{}{}", s, " ".repeat(m))
    }

    fn numeric_field(&self, n: i32, max: u32) -> String {
        let s = n.to_string();
        let l = s.len() as u32;
        if l > max {
            return s[(l - max) as usize..].to_string();
        } else {
            let m = (max - l) as usize;
            if let Some(pad) = MOOV_IO_ACH_STRING_ZEROS.deref().get(&m) {
                return format!("{}{}", pad, s);
            }
            // slow path
            format!("{}{}", "0".repeat(m), s)
        }
    }
}

struct MoovIoAchAddenda05 {
    type_code: String,
    payment_related_information: String,
    sequence_number: i32,
    entry_detail_sequence_number: i32,
    moov_io_ach_converters: Box<MoovIoAchConverters>,
}

struct MoovIoAchConverters {}

