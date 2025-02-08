

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
            let pad = get_pad_string(m, " ");
            s.to_string() + &pad
        }
    }

    fn numeric_field(&self, n: i32, max: usize) -> String {
        let s = n.to_string();
        let l = s.len();
        if l > max {
            s[l - max..].to_string()
        } else {
            let m = max - l;
            let pad = get_pad_string(m, "0");
            pad + &s
        }
    }
}

fn get_pad_string(n: usize, c: &str) -> String {
    iter::repeat(c).take(n).collect::<String>()
}

struct MoovIoAchAddenda12 {
    // ID is a client defined string used as a reference to this record.

    // TypeCode Addenda12 types code '12'
    type_code: String,
    // Originator City & State / Province
    // Data elements City and State / Province  should be separated with an asterisk (*) as a delimiter
    // and the field should end with a backslash (\).
    // For example: San Francisco*CA\
    originator_city_state_province: String,
    // Originator Country & Postal Code
    // Data elements must be separated by an asterisk (*) and must end with a backslash (\)
    // For example: US*10036\
    originator_country_postal_code: String,
    // EntryDetailSequenceNumber contains the ascending sequence number section of the Entry
    // Detail or Corporate Entry Detail Record's trace number This number is
    // the same as the last seven digits of the trace number of the related
    // Entry Detail Record or Corporate Entry Detail Record.
    entry_detail_sequence_number: i32,
    // validator is composed for data validation
    _converters: MoovIoAchConverters,
}

impl fmt::Display for MoovIoAchAddenda12 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.type_code == "Addenda12" {
            return Ok(());
        }

        let mut buf = String::with_capacity(RECORD_LENGTH);
        buf.push_str(ENTRY_ADDENDA_POS);
        buf.push_str(&self.type_code);
        buf.push_str(&self.originator_city_state_province[..35]);
        buf.push_str(&self.originator_country_postal_code[..35]);
        buf.push_str("              ");
        buf.push_str(&self.entry_detail_sequence_number.to_string()[..7]);

        write!(f, "{}", buf)
    }
}

fn main() {}

