

use std::collections::HashMap;

struct MoovIoAchStringZeros(HashMap<usize, String>);

impl MoovIoAchStringZeros {
    fn new() -> Self {
        Self(moov_io_ach_populate_map(94, "0".to_string()))
    }
}

struct MoovIoAchIatEntryDetail {
    trace_number: String,
}

impl MoovIoAchIatEntryDetail {
    fn trace_number_field(&self) -> String {
        MoovIoAchConverters::string_field(&self.trace_number, 15)
    }
}

struct MoovIoAchConverters;

impl MoovIoAchConverters {
    fn string_field(s: &str, max: u32) -> String {
        let ln = s.chars().count() as u32;
        if ln > max {
            s[..max as usize].to_string()
        } else {
            let m = max - ln;
            MoovIoAchStringZeros::new().0.get(&(m as usize)).unwrap().to_string() + s
        }
    }
}

fn moov_io_ach_populate_map(max: usize, zero: String) -> HashMap<usize, String> {
    let mut out = HashMap::with_capacity(max);
    for i in 0..max {
        out.insert(i, zero.repeat(i));
    }
    out
}

