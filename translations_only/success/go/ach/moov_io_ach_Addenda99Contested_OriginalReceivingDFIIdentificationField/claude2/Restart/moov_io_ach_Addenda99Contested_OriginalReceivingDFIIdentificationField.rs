

use std::collections::HashMap;
use std::str;

use once_cell::sync::Lazy;

static MOOV_IO_ACH_STRINGZEROS: Lazy<HashMap<usize, String>> = Lazy::new(|| {
    populate_map(94, "0".to_string())  
});

struct MoovIoAchAddenda99Contested {
    original_receiving_dfi_identification: String,
}

impl MoovIoAchAddenda99Contested {
    fn original_receiving_dfi_identification_field(&self) -> String {
        string_field(&self.original_receiving_dfi_identification, 8)
    }
}

struct MoovIoAchConverters;

fn string_field(s: &String, max: u32) -> String {
    let ln = s.len() as u32;
    if ln > max {
        s[..max as usize].to_string()
    } else {
        let m = max - ln;
        let pad = MOOV_IO_ACH_STRINGZEROS.get(&(m as usize)).unwrap();
        format!("{}{}", pad, s)
    }
}

fn populate_map(max: usize, zero: String) -> HashMap<usize, String> {
    let mut out = HashMap::with_capacity(max);
    for i in 0..max {
        out.insert(i, zero.repeat(i)); 
    }
    out
}

