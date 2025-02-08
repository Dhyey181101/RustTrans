
use std::collections::HashMap;
use std::fmt::Write;
use once_cell::sync::Lazy;

static MOOV_IO_ACH_STRINGZEROS: Lazy<HashMap<i32, String>> = Lazy::new(|| populate_map(94, "0".to_string()));

struct MoovIoAchFileControl {
    entry_hash: i32,
}

impl MoovIoAchFileControl {
    fn entry_hash_field(&self) -> String {
        numeric_field(self.entry_hash, 10)
    }
}

struct MoovIoAchConverters;

fn numeric_field(n: i32, max: u32) -> String {
    let mut s = n.to_string();
    if s.len() as u32 > max {
        s.drain(..s.len() - max as usize);
        s
    } else {
        let mut pad = String::with_capacity(max as usize - s.len());
        for _ in 0..(max - s.len() as u32) {
            pad.push('0');
        }
        pad + &s
    }
}

fn populate_map(max: i32, zero: String) -> HashMap<i32, String> {
    let mut map = HashMap::with_capacity(max as usize);
    for i in 0..max {
        map.insert(i, zero.repeat(i as usize));
    }
    map
}

