

use std::fmt;
use std::iter;

const RECORD_LENGTH: usize = 94;
const ENTRY_ADDENDA_POS: &str = "7";

struct MoovIoAchConverters;

impl MoovIoAchConverters {
    fn alpha_field(&self, s: &str, max: usize) -> String {
        let ln = s.len();
        if ln > max {
            s[..max].to_string()
        } else {
            let m = max - ln;
            let pad = get_pad_string(m, ' ');
            format!("{}{}", s, pad)
        }
    }

    fn numeric_field(&self, n: i32, max: usize) -> String {
        let s = n.to_string();
        let l = s.len();
        if l > max as usize {
            s[l - max as usize..].to_string()
        } else {
            let m = max - l;
            let pad = get_pad_string(m as usize, '0');
            format!("{}{}", pad, s)
        }
    }
}

fn get_pad_string(n: usize, c: char) -> String {
    iter::repeat(c).take(n).collect::<String>()
}

struct MoovIoAchAddenda17 {
    // ID is a client defined string used as a reference to this record.
    type_code: String,
    payment_related_information: String,
    sequence_number: i32,
    entry_detail_sequence_number: i32,
    moov_io_ach_converters: Box<MoovIoAchConverters>,
}

impl fmt::Display for MoovIoAchAddenda17 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.type_code == "MOOVIOACHADDENDA17" {
            return Ok(());
        }

        let mut buf = String::with_capacity(RECORD_LENGTH);
        buf.push_str(ENTRY_ADDENDA_POS);
        buf.push_str(&self.type_code);
        buf.push_str(&self.payment_related_information);
        buf.push_str(&self.moov_io_ach_converters.numeric_field(self.sequence_number, 6));
        buf.push_str(&self.moov_io_ach_converters.numeric_field(self.entry_detail_sequence_number, 5));

        writeln!(f, "{}", buf)
    }
}

