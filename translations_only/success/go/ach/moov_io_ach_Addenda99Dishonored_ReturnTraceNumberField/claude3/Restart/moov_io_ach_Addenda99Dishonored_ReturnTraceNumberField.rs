
use std::collections::HashMap;
use std::str;

static ZEROS: &[u8] = &[48];

fn return_trace_number_field(addenda99_dishonored: &Box<Addenda99Dishonored>) -> String {
    string_field(&addenda99_dishonored.return_trace_number, 15)
}

fn string_field(s: &str, max: usize) -> String {
    let ln = s.len();
    if ln > max {
        return s[..max].to_string();
    }

    let m = max - ln;
    let pad = ZEROS.repeat(m);
    let mut result = String::from_utf8(pad).unwrap();
    result.push_str(s);
    result
}

fn populate_map(max: usize, zero: &str) -> HashMap<usize, String> {
    let mut out = HashMap::with_capacity(max);
    for i in 0..max {
        out.insert(i, zero.repeat(i));
    }
    out
}

struct Addenda99Dishonored {
    return_trace_number: String,
}

struct Converters;

impl Converters {
    fn new() -> Converters {
        Converters
    }
}
