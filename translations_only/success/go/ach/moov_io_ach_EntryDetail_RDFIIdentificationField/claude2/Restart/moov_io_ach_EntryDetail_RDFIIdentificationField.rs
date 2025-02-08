

use std::collections::HashMap;
use std::str;

use once_cell::sync::Lazy;

static MOOV_IO_ACH_STRINGZEROS: Lazy<HashMap<usize, String>> = Lazy::new(|| populate_map(94, "0".to_string()));

struct MoovIoAchEntryDetail {
    rdfi_identification: String,
}

impl MoovIoAchEntryDetail {
    fn rdfi_identification_field(&self) -> String {
        string_field(&self.rdfi_identification, 8)
    }
}

struct MoovIoAchConverters;

fn string_field(s: &str, max: u32) -> String {
    let ln = s.chars().count() as u32;
    if ln > max {
        s[..max as usize].to_string()
    } else {
        let m = max - ln;
        match MOOV_IO_ACH_STRINGZEROS.get(&(m as usize)) {
            Some(pad) => format!("{}{}", pad, s),
            None => "0".repeat(m as usize) + s,
        }
    }
}

fn populate_map(max: usize, zero: String) -> HashMap<usize, String> {
    let mut out = HashMap::with_capacity(max);
    for i in 0..max {
        out.insert(i, "0".repeat(i));
    }
    out
}


