

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
            let pad = get_pad(m as usize);
            return format!("{}{}", pad, &s);
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

struct MoovIoAchAdvBatchControl {
    entry_addenda_count: i32,
}

impl MoovIoAchAdvBatchControl {
    fn new(entry_addenda_count: i32) -> MoovIoAchAdvBatchControl {
        MoovIoAchAdvBatchControl {
            entry_addenda_count,
        }
    }

    fn entry_addenda_count_field(&self) -> String {
        let converters = Box::new(MoovIoAchConverters);
        converters.numeric_field(self.entry_addenda_count, 6)
    }
}

impl fmt::Display for MoovIoAchAdvBatchControl {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "EntryAddendaCount: {}\n",
            self.entry_addenda_count
        )
    }
}

fn main() {
    let batch_control = MoovIoAchAdvBatchControl::new(123);
    println!("{}", batch_control);
    println!(
        "EntryAddendaCountField: {}",
        batch_control.entry_addenda_count_field()
    );
}

