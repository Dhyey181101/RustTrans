

use std::collections::HashMap;

use once_cell::sync::Lazy;

static MOOV_IO_ACH_STRINGZEROS: Lazy<HashMap<usize, String>> = Lazy::new(|| {
    populate_map(94, "0".to_string())  
});

fn populate_map(max: usize, zero: String) -> HashMap<usize, String> {
    let mut out = HashMap::new();
    for i in 0..max {
        out.insert(i, "0".repeat(i));
    }
    out
}

struct MoovIoAchAddenda99Contested {
    contested_return_code: String,
}

impl MoovIoAchAddenda99Contested {
    fn contested_return_code_field(&self) -> String {
        string_field(&self.contested_return_code, 3)
    }
}

fn string_field(s: &String, max: u32) -> String {
    let ln = s.len() as u32;
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

struct MoovIoAchConverters;

