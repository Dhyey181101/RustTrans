
use std::collections::HashMap;
use std::str::Chars;
use once_cell::sync::Lazy;

struct MoovIoAchAddenda98Refused {
    trace_number: String,  
}

impl MoovIoAchAddenda98Refused {
    fn trace_number_field(&self) -> String {
        moov_io_ach_string_field(&self.trace_number, 15)
    }
}

fn moov_io_ach_string_field(s: &String, max: u32) -> String {
    let ln = s.chars().count() as u32;
    if ln > max {
        s[..max as usize].to_string()
    } else {
        let m = max - ln;
        let pad = MOOV_IO_ACH_STRING_ZEROS.get(&(m as usize)).unwrap();
        format!("{}{}", pad, s)
    }
}

static MOOV_IO_ACH_STRING_ZEROS: Lazy<HashMap<usize, String>> = Lazy::new(|| {
    moov_io_ach_populate_map(94, "0".to_string())
});

fn moov_io_ach_populate_map(max: usize, zero: String) -> HashMap<usize, String> {
    let mut out = HashMap::with_capacity(max);
    for i in 0..max {
        out.insert(i, zero.repeat(i));
    }
    out
}

struct MoovIoAchConverters;

