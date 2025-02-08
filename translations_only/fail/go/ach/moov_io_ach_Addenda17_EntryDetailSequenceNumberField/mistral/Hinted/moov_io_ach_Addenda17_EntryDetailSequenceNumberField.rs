

use std::collections::HashMap;
use std::fmt;
use std::str;

const ZEROS: &str = "0";

struct MoovIoAchConverters;

impl MoovIoAchConverters {
    fn numeric_field(&self, n: i32, max: u32) -> String {
        let s = n.to_string();
        let l = s.len() as u32;
        if l > max {
            return s[(l - max) as usize..].to_string();
        } else {
            let m = max - l;
            let pad = MoovIoAchAddenda17::get_pad_string(m as usize);
            return pad + &s;
        }
    }
}

struct MoovIoAchAddenda17 {
    entry_detail_sequence_number: i32,
    moov_io_ach_converters: MoovIoAchConverters,
}

impl MoovIoAchAddenda17 {
    fn entry_detail_sequence_number_field(&self) -> String {
        self.numeric_field(self.entry_detail_sequence_number, 7)
    }

    fn numeric_field(&self, n: i32, max: u32) -> String {
        self.moov_io_ach_converters.numeric_field(n, max)
    }

    fn get_pad_string(n: usize) -> String {
        let mut out = HashMap::new();
        for i in 0..n {
            out.insert(i, "0".repeat(i));
        }
        out[&n].clone()
    }
}

impl fmt::Display for MoovIoAchAddenda17 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "ID: ?, TypeCode: Addenda17, SequenceNumber: {}, EntryDetailSequenceNumber: {}",
            MoovIoAchAddenda17::get_pad_string(11),
            self.entry_detail_sequence_number
        )
    }
}

fn main() {
    let addenda17 = MoovIoAchAddenda17 {
        entry_detail_sequence_number: 1,
        moov_io_ach_converters: MoovIoAchConverters,
    };
    println!("{}", addenda17);
}

