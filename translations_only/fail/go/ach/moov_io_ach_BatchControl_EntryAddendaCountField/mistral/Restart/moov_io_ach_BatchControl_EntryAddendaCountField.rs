

use std::collections::HashMap;
use std::fmt;
use std::str;

const SERVICE_CLASS_CODE_MIXED: &str = "200";
const SERVICE_CLASS_CODE_CREDITS: &str = "220";
const SERVICE_CLASS_CODE_DEBITS: &str = "225";

struct MoovIoAchBatchControl {
    entry_addenda_count: i32,
    // ... other fields elided ...
    converters: Box<MoovIoAchConverters>,
}

impl MoovIoAchBatchControl {
    fn entry_addenda_count_field(&self) -> String {
        self.converters.numeric_field(self.entry_addenda_count, 6)
    }
}

struct MoovIoAchConverters {
    // ... other fields elided ...
}

impl MoovIoAchConverters {
    fn numeric_field(&self, n: i32, max: u32) -> String {
        let s = n.to_string();
        let l = s.len() as u32;
        if l > max {
            s[ (l - max) as usize ..].to_string()
        } else {
            let m = max - l;
            let pad = MoovIoAchBatchControl::get_pad_string(m);
            pad + &s
        }
    }
}

impl MoovIoAchBatchControl {
    fn get_pad_string(mut n: u32) -> String {
        let mut acc = String::new();
        for _ in 0..n {
            acc.push('0');
        }
        acc
    }
}

impl fmt::Display for MoovIoAchBatchControl {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "ID: {}, ServiceClassCode: {:?}, EntryAddendaCount: {}",
            // other fields elided ...
            self.entry_addenda_count,
            SERVICE_CLASS_CODE_MIXED,
            self.entry_addenda_count_field()
        )
    }
}

fn main() {
    let bc = MoovIoAchBatchControl {
        entry_addenda_count: 123,
        // ... other fields elided ...
        converters: Box::new(MoovIoAchConverters {}),
    };
    println!("{}", bc);
}

