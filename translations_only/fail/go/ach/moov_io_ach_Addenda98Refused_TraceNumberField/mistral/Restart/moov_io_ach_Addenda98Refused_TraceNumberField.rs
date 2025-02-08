

use std::collections::HashMap;
use std::fmt;

const ZERO: &str = "0";

struct MoovIoAchConverters;

impl MoovIoAchConverters {
    fn string_field(&self, s: &str, max: usize) -> String {
        let ln = s.len();
        if ln > max {
            s[..max].to_string()
        } else {
            let m = max - ln;
            let pad = Self::get_pad_string(m);
            pad + s
        }
    }

    fn get_pad_string(n: usize) -> String {
        let mut out = HashMap::new();
        for i in 0..=n {
            out.insert(i, "0".repeat(i));
        }
        out[&n].clone()
    }
}

struct MoovIoAchAddenda98Refused {
    trace_number: String,
    moov_io_ach_converters: MoovIoAchConverters,
}

impl MoovIoAchAddenda98Refused {
    fn trace_number_field(&self) -> String {
        self.string_field(&self.trace_number, 15)
    }

    fn string_field(&self, s: &str, max: usize) -> String {
        self.moov_io_ach_converters.string_field(s, max)
    }
}

impl fmt::Display for MoovIoAchAddenda98Refused {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.trace_number_field())
    }
}

