

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
            let start = l - max;
            let mut result = String::new();
            for (i, c) in s.chars().skip(start as usize).enumerate() {
                result.push(c);
                if i == max as usize - 1 {
                    break;
                }
            }
            result
        } else {
            let m = max - l;
            let pad = get_pad_string(m as usize);
            pad + &s
        }
    }
}

fn get_pad_string(n: usize) -> String {
    let mut out = HashMap::new();
    for i in 0..100 {
        out.insert(i, "0".repeat(i));
    }
    out[&n].clone()
}

// Implement Display trait for MoovIoAchBatchControl
impl fmt::Display for MoovIoAchBatchControl {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "EntryAddendaCount: {}, ...",
            self.entry_addenda_count_field()
        )
    }
}

fn main() {
    let batch_control = MoovIoAchBatchControl {
        entry_addenda_count: 123,
        converters: Box::new(MoovIoAchConverters {}),
    };
    println!("{}", batch_control);
}

