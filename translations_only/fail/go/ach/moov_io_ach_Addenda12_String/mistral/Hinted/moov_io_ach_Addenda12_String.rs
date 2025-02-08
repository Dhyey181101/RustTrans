
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
            let pad = get_zeros(m);
            format!("{}{}", s, pad)
        }
    }

    fn numeric_field(&self, n: i32, max: usize) -> String {
        let s = n.to_string();
        if s.len() > max as usize {
            s[s.len() - max as usize..].to_string()
        } else {
            let m = max - s.len();
            let pad = get_zeros(m as usize);
            format!("{}{}", pad, s)
        }
    }
}

fn get_zeros(n: usize) -> String {
    iter::repeat("0").take(n).collect::<String>()
}

struct MoovIoAchAddenda12 {
    // ID is a client defined string used as a reference to this record.
    type_code: String,
    originator_city_state_province: String,
    originator_country_postal_code: String,
    entry_detail_sequence_number: i32,
    moov_io_ach_converters: MoovIoAchConverters,
}

impl fmt::Display for MoovIoAchAddenda12 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            return Ok(());
        }

        let mut buf = String::with_capacity(RECORD_LENGTH);
        buf.push_str(ENTRY_ADDENDA_POS);
        buf.push_str(&self.type_code);
        buf.push_str(&self.originator_city_state_province_field());
        buf.push_str(&self.originator_country_postal_code_field());
        buf.push_str("              ");
        buf.push_str(&self.entry_detail_sequence_number_field());

        write!(f, "{}", buf)
    }
}

impl MoovIoAchAddenda12 {
    fn is_empty(&self) -> bool {
        self.type_code.is_empty()
    }

    fn originator_city_state_province_field(&self) -> String {
        self.moov_io_ach_converters.alpha_field(&self.originator_city_state_province, 35)
    }

    fn originator_country_postal_code_field(&self) -> String {
        self.moov_io_ach_converters.alpha_field(&self.originator_country_postal_code, 35)
    }

    fn entry_detail_sequence_number_field(&self) -> String {
        self.moov_io_ach_converters.numeric_field(self.entry_detail_sequence_number, 7)
    }
}

fn main() {}
