
use std::collections::HashMap;
use once_cell::sync::Lazy;

static MOOV_IO_ACH_STRING_ZEROS: Lazy<Box<HashMap<usize, String>>> = Lazy::new(|| {
    Box::new(populate_map(94))
});

struct Addenda99 {
    trace_number: String,
}

impl Addenda99 {
    fn trace_number_field(&self) -> String {
        string_field(&self.trace_number, 15)
    }
}

fn string_field(s: &str, max: u32) -> String {
    let ln = s.len() as u32;
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

fn populate_map(max: usize) -> HashMap<usize, String> {
    let mut out = HashMap::with_capacity(max);
    for i in 0..max {
        out.insert(i, "0".repeat(i));
    }
    out
}
