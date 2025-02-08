

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
            let pad = MoovIoAchAddenda11::get_zeros(m as usize);
            return pad + &s;
        }
    }
}

struct MoovIoAchAddenda11 {
    entry_detail_sequence_number: i32,
    _converters: MoovIoAchConverters,
}

impl MoovIoAchAddenda11 {
    fn entry_detail_sequence_number_field(&self) -> String {
        self.numeric_field(self.entry_detail_sequence_number, 7)
    }

    fn numeric_field(&self, n: i32, max: u32) -> String {
        MoovIoAchConverters.numeric_field(n, max)
    }

    fn get_zeros(n: usize) -> String {
        let mut out = HashMap::new();
        for i in 0..n {
            out.insert(i, "0".repeat(i));
        }
        out[&(n - 1)].clone()
    }
}

impl fmt::Display for MoovIoAchAddenda11 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "ID: ?, TypeCode: Addenda11 types code '11', Originator Name: ?, Originator Street Address: ?, EntryDetailSequenceNumber: {},",
            self.entry_detail_sequence_number
        )
    }
}

fn main() {
    let addenda11 = MoovIoAchAddenda11 {
        entry_detail_sequence_number: 123,
        _converters: MoovIoAchConverters,
    };
    println!("{}", addenda11);
    println!(
        "EntryDetailSequenceNumberField: {}",
        addenda11.entry_detail_sequence_number_field()
    );
}

