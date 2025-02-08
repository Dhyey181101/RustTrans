

use std::collections::HashMap;
use std::fmt;

#[derive(Default)]
struct MoovIoAchConverters {
    moov_io_ach_string_zeros: HashMap<usize, String>,
}

impl MoovIoAchConverters {
    fn new() -> MoovIoAchConverters {
        let mut moov_io_ach_string_zeros = HashMap::new();
        for i in 0..100 { // max is 94 in Go code, increased for safety
            moov_io_ach_string_zeros.insert(i, "0".repeat(i));
        }
        MoovIoAchConverters { moov_io_ach_string_zeros }
    }

    fn string_field(&self, s: &str, max: usize) -> String {
        let ln = s.len();
        if ln > max {
            s[..max].to_string()
        } else {
            let m = max - ln;
            let pad = self.moov_io_ach_string_zeros.get(&m).cloned().unwrap_or_default();
            format!("{}{}", pad, s)
        }
    }
}

#[derive(Default)]
struct MoovIoAchAddenda02 {
    trace_number: String,
    moov_io_ach_converters: Box<MoovIoAchConverters>,
}

impl MoovIoAchAddenda02 {
    fn trace_number_field(&self) -> String {
        self.moov_io_ach_converters.string_field(&self.trace_number, 15)
    }
}

impl fmt::Debug for MoovIoAchAddenda02 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("MoovIoAchAddenda02")
            .field("trace_number", &self.trace_number)
            .finish()
    }
}

