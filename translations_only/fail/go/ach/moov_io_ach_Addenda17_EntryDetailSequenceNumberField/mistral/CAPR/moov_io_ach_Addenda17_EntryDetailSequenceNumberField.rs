

use std::collections::HashMap;
use std::fmt;

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
    moov_io_ach_converters: Box<MoovIoAchConverters>,
}

impl MoovIoAchAddenda17 {
    fn entry_detail_sequence_number_field(&self) -> String {
        self.moov_io_ach_converters.numeric_field(self.entry_detail_sequence_number, 7)
    }

    fn get_pad_string(n: usize) -> String {
        let mut out = HashMap::new();
        for i in 0..=n {
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
            self.entry_detail_sequence_number,
            self.entry_detail_sequence_number_field()
        )
    }
}

fn main() {
    let addenda17 = MoovIoAchAddenda17 {
        entry_detail_sequence_number: 123,
        moov_io_ach_converters: Box::new(MoovIoAchConverters),
    };
    println!("{}", addenda17);
}

