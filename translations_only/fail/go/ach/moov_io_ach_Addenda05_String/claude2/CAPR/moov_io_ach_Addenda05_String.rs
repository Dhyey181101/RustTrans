
use std::collections::HashMap;
use std::str::FromStr;
use once_cell::sync::Lazy;

static MOOV_IO_ACH_SPACE_ZEROS: Lazy<HashMap<usize, String>> = Lazy::new(|| moov_io_ach_populate_map(94, " "));
static MOOV_IO_ACH_STRING_ZEROS: Lazy<HashMap<usize, String>> = Lazy::new(|| moov_io_ach_populate_map(94, "0"));

const MOOV_IO_ACH_RECORD_LENGTH: usize = 94;
const MOOV_IO_ACH_ENTRY_ADDENDA_POS: &str = "7";

struct MoovIoAchAddenda05 {
    type_code: String,
    payment_related_information: String,
    sequence_number: i32,
    entry_detail_sequence_number: i32,
}

impl MoovIoAchAddenda05 {
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
    let ln = s.len();
    if ln > max {
        s[..max].to_string()
    } else {
        let mut pad = " ".repeat(max - ln);
        pad.push_str(s);
        pad
    }
}

fn numeric_field(n: i32, max: usize) -> String {
    let s = n.to_string();
    let l = s.len();
    
    if l > max {
        s[l - max..].to_string()
    } else {
        let mut pad = "0".repeat(max - l);
        pad.push_str(&s);
        pad
    }
}

fn moov_io_ach_populate_map(max: usize, zero: &str) -> HashMap<usize, String> {
    let mut out = HashMap::with_capacity(max);
    for i in 0..max {
        out.insert(i, zero.repeat(i));
    }
    out
}

