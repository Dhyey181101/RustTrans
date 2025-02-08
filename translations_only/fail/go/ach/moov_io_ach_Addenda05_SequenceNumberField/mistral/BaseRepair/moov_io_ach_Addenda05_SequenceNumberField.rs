

use std::fmt;
use std::collections::HashMap;

struct MoovIoAchConverters;

impl MoovIoAchConverters {
    fn numeric_field(&self, n: i32, max: u32) -> String {
        let s = n.to_string();
        let l = s.len() as u32;
        if l > max {
            s[(l-max) as usize..].to_string()
        } else {
            let m = max - l;
            let pad = MoovIoAchAddenda05::get_zeros(m as usize);
            pad + &s
        }
    }
}

struct MoovIoAchAddenda05 {
    sequence_number: i32,
    entry_detail_sequence_number: i32,
    moov_io_ach_converters: MoovIoAchConverters,
}

impl MoovIoAchAddenda05 {
    fn sequence_number_field(&self) -> String {
        self.numeric_field(self.sequence_number, 4)
    }

    fn numeric_field(&self, n: i32, max: u32) -> String {
        self.moov_io_ach_converters.numeric_field(n, max)
    }

    fn get_zeros(n: usize) -> String {
        let mut out = HashMap::new();
        for i in 0..=n {
            out.insert(i, "0".repeat(i));
        }
        out[&n].clone()
    }
}

impl fmt::Display for MoovIoAchAddenda05 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "ID: ?, TypeCode: 05, PaymentRelatedInformation, SequenceNumber: {}, EntryDetailSequenceNumber: {}",
            self.sequence_number, self.entry_detail_sequence_number
        )
    }
}

