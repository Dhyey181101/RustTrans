
use std::collections::HashMap;
use once_cell::sync::Lazy;

static MOOV_IO_ACH_STRING_ZEROS: Lazy<HashMap<usize, String>> = Lazy::new(|| moov_io_ach_populate_map(94, "0".to_string()));

fn moov_io_ach_adv_entry_detail_advice_routing_number_field(ed: &Box<moov_io_ach_ADVEntryDetail>) -> String {
    moov_io_ach_string_field(ed.advice_routing_number.clone(), 9)
}

fn moov_io_ach_string_field(s: String, max: u32) -> String {
    let ln = s.chars().count() as u32;
    if ln > max {
        return s.chars().take(max as usize).collect();
    }

    let m = (max - ln) as usize;
    if let Some(pad) = MOOV_IO_ACH_STRING_ZEROS.get(&m) {
        return pad.clone() + &s;
    }

    "0".repeat(m) + &s
}

fn moov_io_ach_populate_map(max: usize, zero: String) -> HashMap<usize, String> {
    let mut out = HashMap::with_capacity(max);
    for i in 0..max {
        out.insert(i, zero.repeat(i));
    }
    out
}

struct moov_io_ach_ADVEntryDetail {
    advice_routing_number: String,
}

struct moov_io_ach_converters;
