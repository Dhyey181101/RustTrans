
use std::collections::HashMap;
use once_cell::sync::Lazy as LazyOnce;

pub struct MoovIoAchEntryDetail {
    pub trace_number: String,
}

impl MoovIoAchEntryDetail {
    pub fn trace_number_field(&self) -> String {
        MoovIoAchConverters::string_field(&self.trace_number, 15)
    }
}

pub struct MoovIoAchConverters {}

impl MoovIoAchConverters {
    pub fn string_field(s: &str, max: usize) -> String {
        let ln = s.chars().count();
        if ln > max {
            return s[..max].to_string();
        }

        let m = max - ln;
        let pad = MOOV_IO_ACH_STRING_ZEROS.get(&m).unwrap();
        pad.to_string() + s
    }
}

static MOOV_IO_ACH_STRING_ZEROS: LazyOnce<HashMap<usize, String>> = LazyOnce::new(|| {
    moov_io_ach_populate_map(94, "0")
});

fn moov_io_ach_populate_map(max: usize, zero: &str) -> HashMap<usize, String> {
    let mut out = HashMap::with_capacity(max);
    for i in 0..max {
        out.insert(i, zero.repeat(i));
    }
    out
}

