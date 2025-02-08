

use std::fmt;
use std::collections::HashMap;

const RECORD_LENGTH: usize = 94;
const ENTRY_ADDENDA_POS: &str = "7";

fn moov_io_ach_populate_map(max: usize, zero: char) -> HashMap<usize, String> {
    let mut out = HashMap::new();
    for i in 0..max {
        out.insert(i, "".repeat(i as usize));
    }
    out
}

#[derive(Default)]
struct MoovIoAchConverters;

impl MoovIoAchConverters {
    fn alpha_field(&self, s: &str, max: usize) -> String {
        let ln = s.chars().count();
        if ln > max {
            s[..max].to_string()
        } else {
            let m = max - ln;
            let pad = get_pad(m);
            pad + s
        }
    }

    fn numeric_field(&self, n: i64, max: usize) -> String {
        let s = n.to_string();
        if s.len() > max as usize {
            s[(s.len() as usize) - max..].to_string()
        } else {
            let m = max - s.len();
            let pad = "0".repeat(m);
            pad + &s
        }
    }
}

fn get_pad(n: usize) -> String {
    let mut moov_io_ach_space_zeros = moov_io_ach_populate_map(94, ' ');
    let mut moov_io_ach_string_zeros = moov_io_ach_populate_map(94, '0');
    let pad;
    if let Some(p) = moov_io_ach_space_zeros.get(&n) {
        pad = p.clone();
    } else if let Some(p) = moov_io_ach_string_zeros.get(&n) {
        pad = p.clone();
    } else {
        pad = "0".repeat(n);
    };
    pad
}

#[derive(Default)]
struct MoovIoAchAddenda15 {
    // ID is a client defined string used as a reference to this record.

    // TypeCode Addenda15 types code '15'
    pub type_code: String,
    // Receiver Identification Number contains the accounting number by which the Originator is known to
    // the Receiver for descriptive purposes. NACHA Rules recommend but do not require the RDFI to print
    // the contents of this field on the receiver's statement.
    pub receiver_id_number: String,
    // Receiver Street Address contains the Receiver's physical address
    pub receiver_street_address: String,
    // EntryDetailSequenceNumber contains the ascending sequence number section of the Entry
    // Detail or Corporate Entry Detail Record's trace number This number is
    // the same as the last seven digits of the trace number of the related
    // Entry Detail Record or Corporate Entry Detail Record.
    pub entry_detail_sequence_number: i64,
}

impl PartialEq for MoovIoAchAddenda15 {
    fn eq(&self, other: &Self) -> bool {
        self.type_code == other.type_code
            && self.receiver_id_number == other.receiver_id_number
            && self.receiver_street_address == other.receiver_street_address
            && self.entry_detail_sequence_number == other.entry_detail_sequence_number
    }
}

impl fmt::Display for MoovIoAchAddenda15 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self == &MoovIoAchAddenda15::default() {
            return Ok(());
        }

        let mut buf = String::new();
        buf.push_str(ENTRY_ADDENDA_POS);
        buf.push_str(&self.type_code);
        buf.push_str(&self.receiver_id_number_field());
        buf.push_str(&self.receiver_street_address_field());
        buf.push_str(&"                                  ".to_string());
        buf.push_str(&self.entry_detail_sequence_number_field());
        write!(f, "{}", buf)
    }
}

impl MoovIoAchAddenda15 {
    fn receiver_id_number_field(&self) -> String {
        let moov_io_ach_converters = MoovIoAchConverters::default();
        moov_io_ach_converters.alpha_field(&self.receiver_id_number, 15)
    }

    fn receiver_street_address_field(&self) -> String {
        let moov_io_ach_converters = MoovIoAchConverters::default();
        moov_io_ach_converters.alpha_field(&self.receiver_street_address, 35)
    }

    fn entry_detail_sequence_number_field(&self) -> String {
        let moov_io_ach_converters = MoovIoAchConverters::default();
        moov_io_ach_converters.numeric_field(self.entry_detail_sequence_number, 7)
    }
}

