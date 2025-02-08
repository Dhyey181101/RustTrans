

use std::fmt;
use std::collections::HashMap;
use std::sync::Mutex;
use lazy_static::lazy_static;

const RECORD_LENGTH: usize = 94;
const ENTRY_ADDENDA_POS: &str = "7";

fn moov_io_ach_populate_map(max: usize, zero: char) -> HashMap<usize, String> {
    let mut out = HashMap::new();
    for i in 0..max {
        out.insert(i, "".repeat(i as usize));
    }
    out
}

struct MoovIoAchConverters;

impl MoovIoAchConverters {
    fn alpha_field(&self, s: &str, max: usize) -> String {
        let ln = s.chars().count();
        if ln > max {
            s[..max].to_string()
        } else {
            let m = max - ln;
            let pad = get_pad(m);
            s.to_string() + &pad
        }
    }

    fn numeric_field(&self, n: i64, max: usize) -> String {
        let s = n.to_string();
        let l = s.len();
        if l > max {
            s[l-max..].to_string()
        } else {
            let m = max - l;
            let pad = "0".repeat(m);
            pad + &s
        }
    }
}

struct MoovIoAchAddenda11 {
    // ID is a client defined string used as a reference to this record.

    // TypeCode Addenda11 types code '11'
    type_code: String,
    // Originator Name contains the originators name (your company name / name)
    originator_name: String,
    // Originator Street Address Contains the originators street address (your company's address / your address)
    originator_street_address: String,
    // EntryDetailSequenceNumber contains the ascending sequence number section of the Entry
    // Detail or Corporate Entry Detail Record's trace number This number is
    // the same as the last seven digits of the trace number of the related
    // Entry Detail Record or Corporate Entry Detail Record.
    entry_detail_sequence_number: i64,
    // validator is composed for data validation

    // converters is composed for ACH to GoLang Converters
    converters: MoovIoAchConverters,
}

impl fmt::Display for MoovIoAchAddenda11 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            return Ok(());
        }

        let mut buf = String::with_capacity(RECORD_LENGTH);
        buf.push_str(ENTRY_ADDENDA_POS);
        buf.push_str(&self.type_code);
        buf.push_str(&self.originator_name_field());
        buf.push_str(&self.originator_street_address_field());
        buf.push_str("              ");
        buf.push_str(&self.entry_detail_sequence_number_field());

        write!(f, "{}", buf)
    }
}

impl MoovIoAchAddenda11 {
    fn is_empty(&self) -> bool {
        self.type_code.is_empty()
    }

    fn originator_name_field(&self) -> String {
        self.converters.alpha_field(&self.originator_name, 35)
    }

    fn originator_street_address_field(&self) -> String {
        self.converters.alpha_field(&self.originator_street_address, 35)
    }

    fn entry_detail_sequence_number_field(&self) -> String {
        self.converters.numeric_field(self.entry_detail_sequence_number, 7)
    }
}

fn get_pad(n: usize) -> String {
    let mut space_zeros = MOOV_IO_ACH_SPACE_ZEROS.lock().unwrap();
    let pad = space_zeros[&n].clone();
    drop(space_zeros);
    pad
}

lazy_static! {
    static ref MOOV_IO_ACH_SPACE_ZEROS: Mutex<HashMap<usize, String>> =
        Mutex::new(moov_io_ach_populate_map(94, ' '));
    static ref MOOV_IO_ACH_STRING_ZEROS: Mutex<HashMap<usize, String>> =
        Mutex::new(moov_io_ach_populate_map(94, '0'));
}

