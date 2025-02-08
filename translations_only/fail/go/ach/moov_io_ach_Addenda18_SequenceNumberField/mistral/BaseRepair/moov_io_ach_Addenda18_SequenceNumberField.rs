

use std::collections::HashMap;
use std::fmt;
use std::str;

const ZEROS: &[u8] = b"0000000000";

struct MoovIoAchConverters;

impl MoovIoAchConverters {
    fn numeric_field(&self, n: i32, max: usize) -> String {
        let s = n.to_string();
        let len = s.len();
        if len > max {
            s[len - max..].to_string()
        } else {
            let padding = max - len;
            let pad_str = String::from_utf8_lossy(ZEROS).to_string()[..padding].to_string();
            pad_str + &s
        }
    }
}

struct MoovIoAchAddenda18 {
    sequence_number: i32,
    entry_detail_sequence_number: i32,
    moov_io_ach_converters: MoovIoAchConverters,
}

impl MoovIoAchAddenda18 {
    fn sequence_number_field(&self) -> String {
        self.moov_io_ach_converters.numeric_field(self.sequence_number, 4)
    }
}

impl fmt::Display for MoovIoAchAddenda18 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "SequenceNumber: {}, EntryDetailSequenceNumber: {}",
            self.sequence_number, self.entry_detail_sequence_number
        )
    }
}

fn main() {
    let addenda = MoovIoAchAddenda18 {
        sequence_number: 1,
        entry_detail_sequence_number: 2,
        moov_io_ach_converters: MoovIoAchConverters,
    };
    println!("{}", addenda.sequence_number_field());
}

