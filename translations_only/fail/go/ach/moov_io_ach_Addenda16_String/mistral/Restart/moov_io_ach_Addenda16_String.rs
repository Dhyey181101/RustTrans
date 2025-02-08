

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
            let pad = self.get_pad(m, b' ');
            format!("{}{}", s, pad)
        }
    }

    fn numeric_field(&self, n: i32, max: usize) -> String {
        let s = n.to_string();
        if s.len() > max {
            s[s.len() - max..].to_string()
        } else {
            let m = max - s.len();
            let pad = self.get_pad(m, b'0');
            format!("{}{}", pad, s)
        }
    }

    fn get_pad(&self, n: usize, c: u8) -> String {
        iter::repeat(c as char)
            .take(n)
            .collect::<String>()
    }
}

struct MoovIoAchAddenda16 {
    // ID is a client defined string used as a reference to this record.
    type_code: String,
    receiver_city_state_province: String,
    receiver_country_postal_code: String,
    entry_detail_sequence_number: i32,
    moov_io_ach_converters: Box<MoovIoAchConverters>,
}

impl fmt::Display for MoovIoAchAddenda16 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(
            f,
            "{}{}{}{}{}",
            self.moov_io_ach_converters.alpha_field(&self.type_code, 2),
            self.moov_io_ach_converters.alpha_field(&self.receiver_city_state_province, 35),
            self.moov_io_ach_converters.alpha_field(&self.receiver_country_postal_code, 15),
            self.moov_io_ach_converters.numeric_field(self.entry_detail_sequence_number, 7),
            ENTRY_ADDENDA_POS
        )
    }
}

