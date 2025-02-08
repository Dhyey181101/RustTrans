

use std::collections::HashMap;
use std::fmt;

const SERVICE_CLASS_CODE_MIXED: i32 = 200;
const SERVICE_CLASS_CODE_CREDITS: i32 = 220;
const SERVICE_CLASS_CODE_DEBITS: i32 = 225;

struct MoovIoAchBatchControl {
    entry_hash: i32,
    // ... other fields
    converters: Box<MoovIoAchConverters>,
}

impl MoovIoAchBatchControl {
    fn entry_hash_field(&self) -> String {
        self.numeric_field(self.entry_hash, 10)
    }

    fn numeric_field(&self, n: i32, max: u32) -> String {
        let s = n.to_string();
        let l = s.len() as u32;
        if l > max {
            let start = l - max;
            let chars: Vec<_> = s.chars().take(start as usize).collect();
            let end = s.chars().skip(start as usize).collect::<String>();
            format!("{}{}", chars.into_iter().collect::<String>(), end)
        } else {
            let m = max - l;
            let pad = MoovIoAchBatchControl::get_pad_string(m as usize);
            format!("{}{}", pad, s)
        }
    }

    fn get_pad_string(n: usize) -> String {
        let mut out = HashMap::new();
        for i in 0..=n {
            out.insert(i, "0".repeat(i));
        }
        out[&n].to_string()
    }
}

impl fmt::Display for MoovIoAchBatchControl {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "entry_hash: {},",
            self.entry_hash_field()
            // ... other fields
        )
    }
}

struct MoovIoAchConverters;

fn main() {
    let bc = MoovIoAchBatchControl {
        entry_hash: 12345,
        // ... other fields
        converters: Box::new(MoovIoAchConverters {}),
    };
    println!("{}", bc);
}

