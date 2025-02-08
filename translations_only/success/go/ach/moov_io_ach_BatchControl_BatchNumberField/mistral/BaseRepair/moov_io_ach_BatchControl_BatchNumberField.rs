

use std::collections::HashMap;
use std::fmt;
use std::str;

const SERVICE_CLASS_CODE_MIXED: &str = "200";
const SERVICE_CLASS_CODE_CREDITS: &str = "220";
const SERVICE_CLASS_CODE_DEBITS: &str = "225";

struct MoovIoAchBatchControl {
    batch_number: i32,
    // ... other fields ...
    converters: Box<MoovIoAchConverters>,
}

impl MoovIoAchBatchControl {
    fn batch_number_field(&self) -> String {
        self.converters.numeric_field(self.batch_number, 7)
    }
}

struct MoovIoAchConverters {
    _priv: (),
}

impl MoovIoAchConverters {
    fn numeric_field(&self, n: i32, max: u32) -> String {
        let s = n.to_string();
        let l = s.len() as u32;
        if l > max {
            return s[(l - max) as usize..].to_string();
        }
        let m = max - l;
        let pad = get_pad_string(m as usize);
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

// ... other code ...

