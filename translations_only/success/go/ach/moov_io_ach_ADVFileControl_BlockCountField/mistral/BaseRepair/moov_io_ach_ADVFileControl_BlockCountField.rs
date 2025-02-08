

use std::fmt;
use std::collections::HashMap;
use std::boxed::Box;

struct MoovIoAchConverters;

impl MoovIoAchConverters {
    fn numeric_field(&self, n: i32, max: u32) -> String {
        let s = n.to_string();
        if s.len() as u32 > max {
            s[s.len() - max as usize..].to_string()
        } else {
            let m = (max - s.len() as u32) as usize;
            let pad = get_pad_string(m);
            pad + &s
        }
    }
}

fn get_pad_string(n: usize) -> String {
    let mut out = HashMap::new();
    for i in 0..=n {
        out.insert(i, "0".repeat(i));
    }
    (*out.get(&n).unwrap()).clone()
}

struct MoovIoAchADVFileControl {
    block_count: i32,
    // other fields elided for brevity
}

impl MoovIoAchADVFileControl {
    fn block_count_field(&self) -> String {
        MoovIoAchConverters.numeric_field(self.block_count, 6)
    }
}

fn main() {
    // test code elided for brevity
}

