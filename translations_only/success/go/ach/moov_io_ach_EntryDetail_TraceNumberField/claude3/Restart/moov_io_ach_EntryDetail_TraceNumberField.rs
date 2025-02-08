
use std::collections::HashMap;
use std::str;
use once_cell::sync::Lazy;

static MOOV_IO_ACH_STRING_ZEROS: Lazy<HashMap<usize, String>> = Lazy::new(|| {
    let mut map = HashMap::with_capacity(94);
    for i in 0..94 {
        map.insert(i, "0".repeat(i));
    }
    map
});

fn moov_io_ach_entry_detail_trace_number_field(ed: &Box<moov_io_ach_EntryDetail>) -> String {
    moov_io_ach_string_field(ed.trace_number.as_str(), 15)
}

fn moov_io_ach_string_field(s: &str, max: u32) -> String {
    let ln = s.chars().count() as u32;
    if ln > max {
        return s.chars().take(max as usize).collect();
    }

    let m = (max - ln) as usize;
    if let Some(pad) = MOOV_IO_ACH_STRING_ZEROS.get(&m) {
        return pad.to_owned() + s;
    }

    "0".repeat(m) + s
}

struct moov_io_ach_EntryDetail {
    trace_number: String,
}

struct moov_io_ach_converters;

impl moov_io_ach_converters {
    fn new() -> Self {
        moov_io_ach_converters {}
    }
}
