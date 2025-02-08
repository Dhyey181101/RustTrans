
use std::collections::HashMap;
use std::iter;

use once_cell::sync::Lazy;

static MOOV_IO_ACH_STRINGZEROS: Lazy<HashMap<usize, String>> = Lazy::new(|| populate_map(94, "0".to_string()));

struct MoovIoAchAddenda99Dishonored {
    original_entry_trace_number: String,
}

impl MoovIoAchAddenda99Dishonored {
    fn original_entry_trace_number_field(&self) -> String {
        string_field(&self.original_entry_trace_number, 15)
    }
}

fn string_field(s: &String, max: u32) -> String {
    let ln = s.chars().count() as u32;
    if ln > max {
        s[..max as usize].to_string()
    } else {
        let m = (max - ln) as usize;
        let pad = MOOV_IO_ACH_STRINGZEROS.get(&m).cloned().unwrap_or_else(|| iter::repeat("0").take(m).collect());
        pad + &s
    }
}

fn populate_map(max: usize, zero: String) -> HashMap<usize, String> {
    let mut out = HashMap::with_capacity(max);
    for i in 0..max {
        out.insert(i, iter::repeat("0").take(i).collect());
    }
    out
}

