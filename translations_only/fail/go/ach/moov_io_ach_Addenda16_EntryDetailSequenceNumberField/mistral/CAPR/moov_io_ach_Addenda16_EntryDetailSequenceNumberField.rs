

use std::fmt;
use std::collections::HashMap;

#[derive(Debug)]
struct MoovIoAchConverters;

impl MoovIoAchConverters {
    fn numeric_field(&self, n: i32, max: u32) -> String {
        let s = n.to_string();
        if s.len() as u32 > max {
            return s[(s.len() as usize) - (max as usize) ..].to_string();
        } else {
            let m = max as i32 - s.len() as i32;
            let pad = get_zeros(m as usize);
            return format!("{}{}", pad, &s);
        }
    }
}

fn get_zeros(n: usize) -> String {
    let mut out = String::new();
    for _ in 0 .. n {
        out.push('0');
    }
    out
}

#[derive(Debug)]
struct MoovIoAchAddenda16 {
    entry_detail_sequence_number: i32,
    moov_io_ach_converters: Box<MoovIoAchConverters>,
}

impl MoovIoAchAddenda16 {
    fn entry_detail_sequence_number_field(&self) -> String {
        self.moov_io_ach_converters.numeric_field(self.entry_detail_sequence_number, 7)
    }
}

fn main() {
    let mut moov_io_ach_string_zeros: HashMap<i32, String> = HashMap::new();
    populate_map(&mut moov_io_ach_string_zeros, 94, "0");
}

fn populate_map(moov_io_ach_string_zeros: &mut HashMap<i32, String>, max: i32, zero: &str) {
    for i in 0 .. max {
        let mut out = String::new();
        for _ in 0 .. i {
            out.push_str(zero);
        }
        moov_io_ach_string_zeros.insert(i, out);
    }
}

impl fmt::Display for MoovIoAchAddenda16 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ID: ?, TypeCode: 16, Receiver City & State / Province: ?, Receiver Country & Postal Code: ?, EntryDetailSequenceNumber: {}, converters: {:?}", self.entry_detail_sequence_number, self.moov_io_ach_converters)
    }
}

