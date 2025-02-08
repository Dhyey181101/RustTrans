
use std::collections::HashMap;
use once_cell::sync::Lazy;

static MOOV_IO_ACH_STRING_ZEROS: Lazy<HashMap<usize, String>> = Lazy::new(|| moov_io_ach_populate_map(94, "0"));

struct MoovIoAchAddenda98 {
    original_trace: String,
}

impl MoovIoAchAddenda98 {
    fn original_trace_field(&self) -> String {
        string_field(&self.original_trace, 15)
    }
}

struct MoovIoAchConverters;

fn string_field(s: &String, max: u32) -> String {
    let ln = s.len() as u32;
    if ln > max {
        s[..max as usize].to_string()
    } else {
        let m = max - ln;
        MOOV_IO_ACH_STRING_ZEROS.get(&(m as usize)).unwrap().to_owned() + s
    }
}

fn moov_io_ach_populate_map(max: i32, zero: &str) -> HashMap<usize, String> {
    let mut out = HashMap::with_capacity(max as usize);
    for i in 0..max {
        out.insert(i as usize, zero.repeat(i as usize));
    }
    out
}

