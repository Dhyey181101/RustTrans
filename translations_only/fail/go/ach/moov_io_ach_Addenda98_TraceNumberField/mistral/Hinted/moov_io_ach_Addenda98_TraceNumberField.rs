

use std::collections::HashMap;
use std::fmt;

#[derive(Default)]
struct MoovIoAchConverters;

impl MoovIoAchConverters {
    fn string_field(&self, s: &str, max: usize) -> String {
        let ln = s.len();
        if ln > max {
            s[..max].to_string()
        } else {
            let m = max - ln;
            let pad = get_pad_string(m);
            pad + s
        }
    }
}

fn get_pad_string(n: usize) -> String {
    let mut out = HashMap::new();
    for i in 0..=n {
        out.insert(i, "0".repeat(i));
    }
    out[&n].clone()
}

#[derive(Default)]
struct MoovIoAchAddenda98 {
    trace_number: String,
    moov_io_ach_converters: MoovIoAchConverters,
}

impl MoovIoAchAddenda98 {
    fn trace_number_field(&self) -> String {
        self.moov_io_ach_converters.string_field(&self.trace_number, 15)
    }
}

impl fmt::Display for MoovIoAchAddenda98 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "TraceNumber: {},",
            self.trace_number_field()
        )
    }
}

