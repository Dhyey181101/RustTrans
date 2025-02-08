
use std::collections::HashMap;
use once_cell::sync::Lazy;

static MOOV_IO_ACH_STRING_ZEROS: Lazy<HashMap<usize, String>> = Lazy::new(|| moov_io_ach_populate_map(94, "0".to_string()));

struct Addenda99Dishonored {
    original_entry_trace_number: String,
}

impl Addenda99Dishonored {
    fn original_entry_trace_number_field(&self) -> String {
        self.string_field(&self.original_entry_trace_number, 15)
    }

    fn string_field(&self, s: &str, max: u32) -> String {
        let ln = s.chars().count() as u32;
        if ln > max {
            return s.chars().take(max as usize).collect();
        }

        let m = (max - ln) as usize;
        if let Some(pad) = MOOV_IO_ACH_STRING_ZEROS.get(&m) {
            return pad.clone() + s;
        }

        "0".repeat(m) + s
    }
}

fn moov_io_ach_populate_map(max: usize, zero: String) -> HashMap<usize, String> {
    let mut out = HashMap::with_capacity(max);
    for i in 0..max {
        out.insert(i, "0".repeat(i));
    }
    out
}

struct Converters;

impl Converters {
    fn new() -> Box<Converters> {
        Box::new(Converters)
    }
}
