
use std::collections::HashMap;
use std::sync::Once;
use once_cell::sync::Lazy;

static MOOV_IO_ACH_STRING_ZEROS: Lazy<HashMap<usize, String>> = Lazy::new(|| {
    let mut map = HashMap::with_capacity(94);
    for i in 0..94 {
        map.insert(i, "0".repeat(i));
    }
    map
});

fn moov_io_ach_entry_detail_trace_number_field(ed: &Box<MoovIoAchEntryDetail>) -> String {
    moov_io_ach_converters_string_field(ed.trace_number.clone(), 15)
}

fn moov_io_ach_converters_string_field(s: String, max: u32) -> String {
    let ln = s.chars().count() as u32;
    if ln > max {
        return s.chars().take(max as usize).collect();
    }

    let m = (max - ln) as usize;
    let pad = MOOV_IO_ACH_STRING_ZEROS.get(&m).cloned().unwrap_or_else(|| {
        let mut zeros = String::with_capacity(m);
        zeros.extend(std::iter::repeat('0').take(m));
        zeros
    });
    pad + &s
}

struct MoovIoAchEntryDetail {
    trace_number: String,
    moov_io_ach_converters: Box<MoovIoAchConverters>,
}

struct MoovIoAchConverters {}
