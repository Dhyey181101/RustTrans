

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
            let pad = Self::get_pad_string(m, ' ');
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
            let pad = Self::get_pad_string(m, '0');
            format!("{}{}", pad, s)
        }
    }

    fn get_pad_string(n: usize, c: char) -> String {
        iter::repeat(c).take(n).collect::<String>()
    }
}

#[derive(Default)]
struct MoovIoAchAddenda17 {
    // ID is a client defined string used as a reference to this record.
    type_code: String,
    payment_related_information: String,
    sequence_number: i32,
    entry_detail_sequence_number: i32,
    moov_io_ach_converters: Option<Box<MoovIoAchConverters>>,
}

impl PartialEq for MoovIoAchAddenda17 {
    fn eq(&self, other: &Self) -> bool {
        self.type_code == other.type_code
            && self.payment_related_information == other.payment_related_information
            && self.sequence_number == other.sequence_number
            && self.entry_detail_sequence_number == other.entry_detail_sequence_number
    }
}

impl fmt::Display for MoovIoAchAddenda17 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self == &MoovIoAchAddenda17::default() {
            return Ok(());
        }

        let mut buf = String::with_capacity(RECORD_LENGTH);
        buf.push_str(ENTRY_ADDENDA_POS);
        buf.push_str(&self.type_code);
        buf.push_str(&self.payment_related_information[..80]);
        if let Some(ref c) = self.moov_io_ach_converters {
            buf.push_str(&c.numeric_field(self.sequence_number, 4));
            buf.push_str(&c.numeric_field(self.entry_detail_sequence_number, 7));
        }

        write!(f, "{}", buf)
    }
}

impl MoovIoAchAddenda17 {
    fn payment_related_information_field(&self) -> String {
        self.moov_io_ach_converters.as_ref().map_or("".to_string(), |c| {
            c.alpha_field(&self.payment_related_information, 80)
        })
    }

    fn sequence_number_field(&self) -> String {
        self.moov_io_ach_converters.as_ref().map_or("".to_string(), |c| {
            c.numeric_field(self.sequence_number, 4)
        })
    }

    fn entry_detail_sequence_number_field(&self) -> String {
        self.moov_io_ach_converters.as_ref().map_or("".to_string(), |c| {
            c.numeric_field(
                self.entry_detail_sequence_number,
                7,
            )
        })
    }
}

fn main() {}

