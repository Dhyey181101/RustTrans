
use std::collections::HashMap;
use std::fmt;
use once_cell::sync::Lazy;

static MOOV_IO_ACH_STRING_ZEROS: Lazy<HashMap<usize, String>> = Lazy::new(|| populate_map(94, "0".to_string()));

struct MoovIoAchAdvFileControl {
    entry_hash: i32,
}

impl MoovIoAchAdvFileControl {
    fn entry_hash_field(&self) -> String {
        numeric_field(self.entry_hash, 10)
    }
}

fn numeric_field(n: i32, max: u32) -> String {
    let s = n.to_string();
    let l = s.len();

    if l as u32 > max {
        s[l - max as usize..].to_string()
    } else {
        let m = (max - l as u32) as usize;
        MOOV_IO_ACH_STRING_ZEROS.get(&m).cloned().unwrap_or_else(|| "0".repeat(m)) + &s
    }
}

fn populate_map(max: usize, zero: String) -> HashMap<usize, String> {
    let mut map = HashMap::with_capacity(max);
    for i in 0..max {
        map.insert(i, zero.repeat(i));
    }
    map
}

