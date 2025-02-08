

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
            let pad_str = String::from_utf8(ZEROS[..padding].to_vec()).unwrap();
            pad_str + &s
        }
    }
}

struct MoovIoAchAddenda18 {
    sequence_number: i32,
    // ... other fields ...
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
            "SequenceNumber: {}, SequenceNumberField: {}",
            self.sequence_number, self.sequence_number_field()
        )
    }
}

fn main() {
    let addenda = MoovIoAchAddenda18 {
        sequence_number: 12345,
        // ... other fields ...
        moov_io_ach_converters: MoovIoAchConverters,
    };
    println!("{}", addenda);
}

