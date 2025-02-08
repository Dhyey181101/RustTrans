

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
            let pad = get_pad(m as usize);
            return format!("{}{}", pad, s);
        }
    }
}

fn get_pad(n: usize) -> String {
    let mut out = HashMap::new();
    for i in 0..=n {
        out.insert(i, "0".repeat(i));
    }
    out[&n].to_string()
}

struct MoovIoAchAddenda12 {
    entry_detail_sequence_number: i32,
    moov_io_ach_converters: Box<MoovIoAchConverters>,
}

impl MoovIoAchAddenda12 {
    fn entry_detail_sequence_number_field(&self) -> String {
        self.moov_io_ach_converters.numeric_field(self.entry_detail_sequence_number, 7)
    }
}

impl fmt::Display for MoovIoAchAddenda12 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "ID: {}\nTypeCode: 12\nOriginator City & State / Province: {}\nOriginator Country & Postal Code: {}\nEntryDetailSequenceNumber: {}\n",
            "".to_string(),
            "".to_string(),
            "".to_string(),
            self.entry_detail_sequence_number
        )
    }
}

fn main() {
    let addenda12 = MoovIoAchAddenda12 {
        entry_detail_sequence_number: 12345,
        moov_io_ach_converters: Box::new(MoovIoAchConverters),
    };
    println!("{}", addenda12);
    println!("EntryDetailSequenceNumberField: {}", addenda12.entry_detail_sequence_number_field());
}

