

use std::collections::HashMap;
use std::fmt;

#[derive(Default)]
struct MoovIoAchConverters {
    moov_io_ach_string_zeros: HashMap<usize, String>,
}

impl MoovIoAchConverters {
    fn string_field(&self, s: &str, max: usize) -> String {
        let ln = s.len();
        if ln > max {
            s[..max].to_string()
        } else {
            let m = max - ln;
            let pad = self.moov_io_ach_string_zeros.get(&m).cloned().unwrap_or_else(|| "0".repeat(m));
            pad + s
        }
    }
}

impl MoovIoAchConverters {
    fn moov_io_ach_populate_map(max: usize, zero: char) -> HashMap<usize, String> {
        let mut out = HashMap::new();
        for i in 0..max {
            out.insert(i, "0".repeat(i));
        }
        out
    }

    fn new() -> Self {
        MoovIoAchConverters {
            moov_io_ach_string_zeros: Self::moov_io_ach_populate_map(94, '0'),
        }
    }
}

#[derive(Default)]
struct MoovIoAchAddenda99 {
    trace_number: String,
    moov_io_ach_converters: Box<MoovIoAchConverters>,
}

impl MoovIoAchAddenda99 {
    fn trace_number_field(&self) -> String {
        self.string_field(&self.trace_number, 15)
    }

    fn string_field(&self, s: &str, max: usize) -> String {
        self.moov_io_ach_converters.string_field(s, max)
    }
}

impl fmt::Display for MoovIoAchAddenda99 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "TraceNumber: {}",
            self.trace_number_field()
        )
    }
}

