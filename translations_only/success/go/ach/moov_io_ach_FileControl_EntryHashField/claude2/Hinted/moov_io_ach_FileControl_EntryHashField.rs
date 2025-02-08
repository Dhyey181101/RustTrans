
use std::collections::HashMap;
use std::fmt::Write;
use once_cell::sync::Lazy;

static MOOV_IO_ACH_STRING_ZEROS: Lazy<HashMap<usize, String>> = Lazy::new(|| populate_map(94, "0".to_string()));

struct MoovIoAchFileControl {
    entry_hash: i32,
}

impl MoovIoAchFileControl {
    fn entry_hash_field(&self) -> String {
        numeric_field(self.entry_hash, 10)
    }
}

fn numeric_field(n: i32, max: u32) -> String {
    let mut s = n.to_string();
    let l = s.len();
    
    if l > max as usize {
        s.drain(..l - max as usize);
        s
    } else {
        let mut pad = String::new();
        if let Some(p) = MOOV_IO_ACH_STRING_ZEROS.get(&(max as usize - l)) {
            pad = p.clone();
        } else {
            write!(pad, "{:0width$}", 0, width=max as usize - l).unwrap();
        }
        format!("{}{}", pad, s)
    }
}

fn populate_map(max: usize, zero: String) -> HashMap<usize, String> {
    let mut map = HashMap::new();
    for i in 0..max {
        map.insert(i, zero.repeat(i));
    }
    map
}

