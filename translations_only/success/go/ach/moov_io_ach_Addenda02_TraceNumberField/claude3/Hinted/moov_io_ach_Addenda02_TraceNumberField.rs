
use std::collections::HashMap;
use std::str;
use once_cell::sync::Lazy;

static MOOV_IO_ACH_STRING_ZEROS: Lazy<HashMap<usize, String>> = Lazy::new(|| populate_map(94, "0"));

fn trace_number_field(addenda02: &Box<Addenda02>) -> String {
    addenda02.string_field(&addenda02.trace_number, 15)
}

fn string_field(s: &str, max: u32) -> String {
    let ln = s.chars().count() as u32;
    if ln > max {
        return s.chars().take(max as usize).collect();
    }

    let m = (max - ln) as usize;
    let pad = MOOV_IO_ACH_STRING_ZEROS.get(&m);
    if let Some(pad) = pad {
        return pad.to_owned() + s;
    }

    "0".repeat(m) + s
}

fn populate_map(max: usize, zero: &str) -> HashMap<usize, String> {
    let mut out = HashMap::with_capacity(max);
    for i in 0..max {
        out.insert(i, zero.repeat(i));
    }
    out
}

struct Addenda02 {
    trace_number: String,
}

impl Addenda02 {
    fn string_field(&self, s: &str, max: u32) -> String {
        string_field(s, max)
    }
}
