

use std::collections::HashMap;
use std::fmt;
use std::str;

const SERVICE_CLASS_CODE_MIXED: &str = "200";
const SERVICE_CLASS_CODE_CREDITS: &str = "220";
const SERVICE_CLASS_CODE_DEBITS: &str = "225";

struct MoovIoAchBatchControl {
    entry_addenda_count: i32,
    // ... other fields omitted for brevity
    converters: Box<MoovIoAchConverters>,
}

impl MoovIoAchBatchControl {
    fn entry_addenda_count_field(&self) -> String {
        self.converters.numeric_field(self.entry_addenda_count, 6)
    }
}

struct MoovIoAchConverters;

impl MoovIoAchConverters {
    fn numeric_field(&self, n: i32, max: u32) -> String {
        let s = n.to_string();
        let l = s.len() as u32;
        if l > max {
            let start = max - l;
            let end = s.len();
            format!("{}{}", &s[start as usize..], "")
        } else {
            let m = max - l;
            let pad = MoovIoAchBatchControl::get_pad_string(m as u32);
            format!("{}{}", pad, &s)
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
            "EntryAddendaCount: {}, ServiceClassCode: {}",
            self.entry_addenda_count, SERVICE_CLASS_CODE_MIXED
        )
    }
}

fn main() {
    let bc = MoovIoAchBatchControl {
        entry_addenda_count: 123,
        converters: Box::new(MoovIoAchConverters),
    };
    println!("{}", bc);
}

