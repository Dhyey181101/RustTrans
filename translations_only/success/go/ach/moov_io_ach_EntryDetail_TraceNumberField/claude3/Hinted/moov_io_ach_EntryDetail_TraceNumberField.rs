
use std::collections::HashMap;

static mut MOOV_IO_ACH_STRING_ZEROS: Option<HashMap<usize, String>> = None;

fn moov_io_ach_populate_map(max: usize, zero: &str) -> HashMap<usize, String> {
    let mut out = HashMap::with_capacity(max);
    for i in 0..max {
        out.insert(i, zero.repeat(i));
    }
    out
}

fn moov_io_ach_string_field(s: &str, max: u32) -> String {
    let ln = s.chars().count() as u32;
    if ln > max {
        return s.chars().take(max as usize).collect();
    }

    let m = (max - ln) as usize;
    unsafe {
        if MOOV_IO_ACH_STRING_ZEROS.is_none() {
            MOOV_IO_ACH_STRING_ZEROS = Some(moov_io_ach_populate_map(94, "0"));
        }
        if let Some(pad) = MOOV_IO_ACH_STRING_ZEROS.as_ref().unwrap().get(&m) {
            return pad.clone() + s;
        }
    }
    "0".repeat(m) + s
}

fn trace_number_field(ed: &Box<moov_io_ach_EntryDetail>) -> String {
    moov_io_ach_string_field(&ed.trace_number, 15)
}

struct moov_io_ach_EntryDetail {
    trace_number: String,
}

struct moov_io_ach_converters {}
