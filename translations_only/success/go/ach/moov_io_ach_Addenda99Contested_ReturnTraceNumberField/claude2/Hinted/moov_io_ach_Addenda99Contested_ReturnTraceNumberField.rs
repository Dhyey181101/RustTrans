
use std::collections::HashMap;
use once_cell::sync::Lazy;

static MOOV_IO_ACH_STRING_ZEROS: Lazy<HashMap<usize, String>> = Lazy::new(|| populate_map(94, "0".to_string()));

struct MoovIoAchAddenda99Contested {
    return_trace_number: String,
}

impl MoovIoAchAddenda99Contested {
    fn return_trace_number_field(&self) -> String {
        string_field(&self.return_trace_number, 15)
    }
}

struct MoovIoAchConverters;

fn string_field(s: &String, max: u32) -> String {
    let ln = s.len() as u32;
    if ln > max {
        s[..max as usize].to_string()
    } else {
        let m = max - ln;
        match MOOV_IO_ACH_STRING_ZEROS.get(&(m as usize)) {
            Some(pad) => pad.to_owned() + s,
            None => "0".repeat(m as usize) + s,
        }
    }
}

fn populate_map(max: usize, zero: String) -> HashMap<usize, String> {
    let mut out = HashMap::with_capacity(max);
    for i in 0..max {
        out.insert(i, zero.repeat(i));
    }
    out
}

