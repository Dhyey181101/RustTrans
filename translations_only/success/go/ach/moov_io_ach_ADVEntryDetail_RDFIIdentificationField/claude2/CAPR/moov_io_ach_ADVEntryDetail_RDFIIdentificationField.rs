

use std::collections::HashMap;

struct MoovIoAchAdvEntryDetail {
    rdfi_identification: String,
}

impl MoovIoAchAdvEntryDetail {
    fn rdfi_identification_field(&self) -> String {
        string_field(&self.rdfi_identification, 8)
    }
}

struct MoovIoAchConverters;

fn string_field(s: &str, max: u32) -> String {
    let ln = s.len() as u32;
    if ln > max {
        s[..max as usize].to_string()
    } else {
        let m = max - ln;
        let pad = "0".repeat(m as usize);
        format!("{}{}", pad, s)
    }
}

fn moov_io_ach_populate_map(max: i32, zero: String) -> HashMap<usize, String> {
    let mut out = HashMap::with_capacity(max as usize);
    for i in 0..max {
        out.insert(i as usize, zero.repeat(i as usize));
    }
    out
}

