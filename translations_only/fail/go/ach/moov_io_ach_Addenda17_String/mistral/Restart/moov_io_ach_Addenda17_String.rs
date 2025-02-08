
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
    moov_io_ach_converters: MoovIoAchConverters,
}

impl MoovIoAchAddenda17 {
    fn new() -> MoovIoAchAddenda17 {
        MoovIoAchAddenda17 {
            type_code: String::new(),
            payment_related_information: String::new(),
            sequence_number: 0,
            entry_detail_sequence_number: 0,
            moov_io_ach_converters: MoovIoAchConverters,
        }
    }

    fn string(&self) -> String {
        let mut buf = String::with_capacity(RECORD_LENGTH);
        buf.push_str(ENTRY_ADDENDA_POS);
        buf.push_str(&self.type_code);
        buf.push_str(&self.payment_related_information_field());
        buf.push_str(&self.sequence_number_field());
        buf.push_str(&self.entry_detail_sequence_number_field());
        buf
    }

    fn payment_related_information_field(&self) -> String {
        self.moov_io_ach_converters.alpha_field(&self.payment_related_information, 80)
    }

    fn sequence_number_field(&self) -> String {
        self.moov_io_ach_converters.numeric_field(self.sequence_number, 4)
    }

    fn entry_detail_sequence_number_field(&self) -> String {
        self.moov_io_ach_converters.numeric_field(self.entry_detail_sequence_number, 7)
    }
}

impl fmt::Display for MoovIoAchAddenda17 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.string())
    }
}
