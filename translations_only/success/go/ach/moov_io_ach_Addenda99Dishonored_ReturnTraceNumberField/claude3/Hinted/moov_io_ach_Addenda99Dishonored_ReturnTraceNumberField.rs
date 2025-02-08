
use std::collections::HashMap;
use std::str;

static ZEROS: &[u8] = &[48];

fn return_trace_number_field(addenda99_dishonored: &Addenda99Dishonored) -> String {
    string_field(&addenda99_dishonored.return_trace_number, 15)
}

fn string_field(s: &str, max: usize) -> String {
    let len = s.len();
    if len > max {
        return s[..max].to_string();
    }

    let m = max - len;
    let mut pad = String::with_capacity(max);
    unsafe {
        pad.as_mut_vec().extend(ZEROS.iter().cycle().take(m));
    }
    pad.push_str(s);
    pad
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

impl Addenda99Dishonored {
    fn new(return_trace_number: String) -> Box<Self> {
        Box::new(Addenda99Dishonored {
            return_trace_number,
        })
    }
}
