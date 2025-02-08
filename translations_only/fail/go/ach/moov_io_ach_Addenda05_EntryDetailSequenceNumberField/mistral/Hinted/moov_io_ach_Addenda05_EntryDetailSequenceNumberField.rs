

use std::fmt;
use std::collections::HashMap;

struct MoovIoAchConverters;

impl MoovIoAchConverters {
    fn numeric_field(&self, n: i32, max: u32) -> String {
        let s = n.to_string();
        if s.len() as u32 > max {
            let max_usize = max as usize;
            return s[(s.len() as usize) - max_usize ..].to_string();
        } else {
            let m = max as i32 - s.len() as i32;
            let pad = MoovIoAchAddenda05::get_pad_string(m);
            return pad + &s;
        }
    }
}

struct MoovIoAchAddenda05 {
    entry_detail_sequence_number: i32,
    moov_io_ach_converters: Box<MoovIoAchConverters>,
}

impl MoovIoAchAddenda05 {
    fn entry_detail_sequence_number_field(&self) -> String {
        self.moov_io_ach_converters.numeric_field(self.entry_detail_sequence_number, 7)
    }

    fn get_pad_string(n: i32) -> String {
        let mut out = HashMap::new();
        for i in 0..=n {
            out.insert(i, "0".repeat(i as usize));
        }
        out[&n].clone()
    }
}

impl fmt::Display for MoovIoAchAddenda05 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ID: ?, TypeCode: 05, PaymentRelatedInformation, SequenceNumber: {}, EntryDetailSequenceNumber: {}", self.entry_detail_sequence_number, self.entry_detail_sequence_number,)
    }
}

fn main() {
    let addenda05 = MoovIoAchAddenda05 {
        entry_detail_sequence_number: 1,
        moov_io_ach_converters: Box::new(MoovIoAchConverters),
    };
    println!("{}", addenda05);
    println!("EntryDetailSequenceNumberField: {}", addenda05.entry_detail_sequence_number_field());
}

