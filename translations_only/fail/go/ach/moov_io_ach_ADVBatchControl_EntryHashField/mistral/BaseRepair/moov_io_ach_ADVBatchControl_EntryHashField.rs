

use std::collections::HashMap;
use std::fmt;

const ZEROS: &str = "0";

struct MoovIoAchAdvBatchControl {
    entry_hash: i32,
    // ... other fields ...
    converters: Box<MoovIoAchConverters>,
}

impl MoovIoAchAdvBatchControl {
    fn entry_hash_field(&self) -> String {
        self.converters.numeric_field(self.entry_hash, 10)
    }
}

struct MoovIoAchConverters {
    numeric_field_map: HashMap<u32, String>,
}

impl MoovIoAchConverters {
    fn numeric_field(&self, n: i32, max: u32) -> String {
        let s = n.to_string();
        let l = s.len() as u32;
        if l > max {
            let start = (l - max) as usize;
            s[start..].to_string()
        } else {
            let m = max - l;
            let pad = MoovIoAchAdvBatchControl::get_pad_string(m as usize);
            pad + &s
        }
    }
}

impl MoovIoAchAdvBatchControl {
    fn get_pad_string(len: usize) -> String {
        let mut out = HashMap::new();
        for i in 0..100 {
            out.insert(i, ZEROS.repeat(i));
        }
        out[&len].clone()
    }
}

impl fmt::Display for MoovIoAchAdvBatchControl {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "EntryHash: {}, ...",
            self.entry_hash_field()
        )
    }
}

fn main() {
    let batch_control = MoovIoAchAdvBatchControl {
        entry_hash: 12345,
        converters: Box::new(MoovIoAchConverters {
            numeric_field_map: HashMap::new(),
        }),
    };
    println!("{}", batch_control);
}

