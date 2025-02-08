

use std::collections::HashMap;
use std::fmt;

struct MoovIoAchConverters {
    moov_io_ach_string_zeros: HashMap<usize, String>,
}

impl MoovIoAchConverters {
    fn new() -> MoovIoAchConverters {
        MoovIoAchConverters {
            moov_io_ach_string_zeros: MoovIoAchConverters::moov_io_ach_populate_map(MOOV_IO_ACH_MAX, "0"),
        }
    }

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

    fn moov_io_ach_populate_map(max: usize, zero: &str) -> HashMap<usize, String> {
        let mut out = HashMap::new();
        for i in 0..max {
            out.insert(i, zero.repeat(i));
        }
        out
    }
}

const MOOV_IO_ACH_MAX: usize = 94;

struct MoovIoAchAddenda99 {
    trace_number: String,
    moov_io_ach_converters: MoovIoAchConverters,
}

impl Default for MoovIoAchAddenda99 {
    fn default() -> MoovIoAchAddenda99 {
        MoovIoAchAddenda99 {
            trace_number: String::new(),
            moov_io_ach_converters: MoovIoAchConverters::new(),
        }
    }
}

impl MoovIoAchAddenda99 {
    fn trace_number_field(&self) -> String {
        self.moov_io_ach_converters.string_field(&self.trace_number, 15)
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
            self.trace_number
        )
    }
}

