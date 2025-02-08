
use std::collections::HashMap;
use std::str::FromStr;
use once_cell::sync::Lazy;

static MOV_IO_ACH_STRING_ZEROS: Lazy<HashMap<i32, String>> = Lazy::new(|| populate_map(94, "0".to_string()));

struct MovIoAchAdvFileControl {
    entry_hash: i32,
}

impl MovIoAchAdvFileControl {
    fn entry_hash_field(&self) -> String {
        numeric_field(self.entry_hash, 10)
    }
}

struct MovIoAchConverters;

fn numeric_field(n: i32, max: u32) -> String {
    let s = n.to_string();
    if s.len() as u32 > max {
        s.chars().skip(s.len() as usize - max as usize).collect()
    } else {
        let m = max - s.len() as u32;
        let pad = MOV_IO_ACH_STRING_ZEROS.get(&(m as i32)).unwrap();
        format!("{}{}", pad, s)
    }
}

fn populate_map(max: i32, zero: String) -> HashMap<i32, String> {
    let mut out = HashMap::new();
    for i in 0..max {
        out.insert(i, zero.repeat(i as usize));
    }
    out
}

