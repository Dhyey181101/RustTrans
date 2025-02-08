
use std::collections::HashMap;
use std::fmt;

const MOOV_IO_ACH_RECORD_LENGTH: usize = 94;
const MOOV_IO_ACH_ENTRY_ADDENDA_POS: &str = "7";

lazy_static! {
    static ref MOOV_IO_ACH_SPACE_ZEROS: HashMap<usize, String> = moov_io_ach_populate_map(94, " ");
    static ref MOOV_IO_ACH_STRING_ZEROS: HashMap<usize, String> = moov_io_ach_populate_map(94, "0");
}

fn moov_io_ach_populate_map(max: usize, zero: &str) -> HashMap<usize, String> {
    let mut out = HashMap::with_capacity(max);
    for i in 0..max {
        out.insert(i, zero.repeat(i));
    }
    out
}

struct MoovIoAchAddenda17 {
    type_code: String,
    payment_related_information: String,
    sequence_number: i32,
    entry_detail_sequence_number: i32,
}

impl MoovIoAchAddenda17 {
    fn new(type_code: String, payment_related_information: String, sequence_number: i32, entry_detail_sequence_number: i32) -> Box<Self> {
        Box::new(Self {
            type_code,
            payment_related_information,
            sequence_number,
            entry_detail_sequence_number,
        })
    }

    fn string(&self) -> String {
        let mut buf = String::with_capacity(MOOV_IO_ACH_RECORD_LENGTH);
        buf.push_str(MOOV_IO_ACH_ENTRY_ADDENDA_POS);
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
        s.chars().take(max).collect()
    } else {
        let m = max - ln;
        if let Some(pad) = MOOV_IO_ACH_SPACE_ZEROS.get(&m) {
            format!("{}{}", s, pad)
        } else {
            format!("{}{}", s, " ".repeat(max - ln))
        }
    }
}

fn numeric_field(n: i32, max: usize) -> String {
    let s = n.to_string();
    let l = s.len();
    if l > max {
        s.chars().skip(l - max).collect()
    } else {
        let m = max - l;
        if let Some(pad) = MOOV_IO_ACH_STRING_ZEROS.get(&m) {
            format!("{}{}", pad, s)
        } else {
            format!("{}{}", "0".repeat(max - l), s)
        }
    }
}

#[macro_use]
extern crate lazy_static;

fn main() {
    let addenda17 = MoovIoAchAddenda17::new(
        "TypeCode".to_string(),
        "PaymentRelatedInformation".to_string(),
        1,
        1234567,
    );
    println!("{}", addenda17.string());
}
