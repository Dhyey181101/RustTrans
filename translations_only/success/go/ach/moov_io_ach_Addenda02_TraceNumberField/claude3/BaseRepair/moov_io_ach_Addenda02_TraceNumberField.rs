
use std::collections::HashMap;
use std::str;
use once_cell::sync::Lazy;

static ZEROS: Lazy<HashMap<usize, String>> = Lazy::new(|| {
    let mut out = HashMap::with_capacity(94);
    for i in 0..94 {
        out.insert(i, "0".repeat(i));
    }
    out
});

fn trace_number_field(addenda02: &Addenda02) -> String {
    string_field(&addenda02.trace_number, 15)
}

fn string_field(s: &str, max: u32) -> String {
    let ln = s.chars().count() as u32;
    if ln > max {
        return s.chars().take(max as usize).collect();
    }

    let m = (max - ln) as usize;
    if let Some(pad) = ZEROS.get(&m) {
        return pad.to_owned() + s;
    }

    // slow path
    "0".repeat(m) + s
}

struct Addenda02 {
    trace_number: String,
    converters: Box<Converters>,
}

struct Converters;

impl Converters {
    fn new() -> Box<Converters> {
        Box::new(Converters)
    }
}
