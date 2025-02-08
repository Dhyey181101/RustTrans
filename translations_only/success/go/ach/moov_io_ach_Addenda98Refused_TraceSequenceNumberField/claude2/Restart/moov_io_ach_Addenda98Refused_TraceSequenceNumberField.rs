
use std::collections::HashMap;

use once_cell::sync::Lazy;

static MOOV_IO_ACH_STRING_ZEROS: Lazy<HashMap<usize, String>> = Lazy::new(|| {
    moov_io_ach_populate_map(94, "0")
});

struct MoovIoAchAddenda98Refused {
    trace_sequence_number: String,
    moov_io_ach_converters: Box<MoovIoAchConverters>,
}

struct MoovIoAchConverters;

impl MoovIoAchAddenda98Refused {
    fn trace_sequence_number_field(&self) -> String {
        self.moov_io_ach_converters.string_field(&self.trace_sequence_number, 7)
    }
}

impl MoovIoAchConverters {
    fn string_field(&self, s: &String, max: u32) -> String {
        let ln = s.len() as u32;
        if ln > max {
            s[..max as usize].to_string()
        } else {
            let m = max - ln;
            let pad = MOOV_IO_ACH_STRING_ZEROS.get(&(m as usize)).unwrap().clone();
            pad + s
        }
    }
}

fn moov_io_ach_populate_map(max: usize, zero: &str) -> HashMap<usize, String> {
    let mut out = HashMap::with_capacity(max);
    for i in 0..max {
        out.insert(i, zero.repeat(i));
    }
    out
}

