
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

fn moov_io_ach_populate_map(max: usize, zero: &str) -> HashMap<usize, String> {
    let mut out = HashMap::with_capacity(max);
    for i in 0..max {
        out.insert(i, zero.repeat(i));
    }
    out
}

fn moov_io_ach_string_field(s: &str, max: usize) -> String {
    let ln = s.chars().count();
    if ln > max {
        return s.chars().take(max).collect();
    }

    let m = max - ln;
    if let Some(pad) = MOOV_IO_ACH_STRING_ZEROS.get(&m) {
        return pad.clone() + s;
    }

    let mut pad = String::with_capacity(m);
    pad.push_str(&"0".repeat(m));
    pad + s
}

struct MoovIoAchEntryDetail {
    identification_number: String,
}

impl MoovIoAchEntryDetail {
    fn shr_document_reference_number_field(&self) -> String {
        moov_io_ach_string_field(&self.identification_number[4..15], 11)
    }
}

struct MoovIoAchConverters {}

impl MoovIoAchConverters {
    fn string_field(&self, s: &str, max: usize) -> String {
        moov_io_ach_string_field(s, max)
    }
}
