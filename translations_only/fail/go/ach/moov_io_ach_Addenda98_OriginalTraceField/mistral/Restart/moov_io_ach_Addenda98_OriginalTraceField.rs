
use std::collections::HashMap;
use std::fmt;

struct MoovIoAchConverters {
    moov_io_ach_string_zeros: HashMap<usize, String>,
}

impl MoovIoAchConverters {
    fn new() -> Self {
        MoovIoAchConverters {
            moov_io_ach_string_zeros: HashMap::new(),
        }
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

struct MoovIoAchAddenda98 {
    original_trace: String,
    moov_io_ach_converters: Box<MoovIoAchConverters>,
}

impl MoovIoAchAddenda98 {
    fn new() -> Self {
        MoovIoAchAddenda98 {
            original_trace: String::new(),
            moov_io_ach_converters: Box::new(MoovIoAchConverters::new()),
        }
    }
}
