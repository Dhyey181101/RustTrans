

use std::collections::HashMap;
use std::fmt;

struct MoovIoAchConverters {
    moov_io_ach_string_zeros: HashMap<usize, u8>,
}

impl MoovIoAchConverters {
    fn string_field(&self, s: &str, max: usize) -> String {
        let ln = s.len();
        if ln > max {
            s[0..max].to_string()
        } else {
            let m = max - ln;
            let pad = *self
                .moov_io_ach_string_zeros
                .get(&m)
                .unwrap_or(&0);
            let pad_str = "0".repeat(pad as usize);
            format!("{}{}", pad_str, s)
        }
    }
}

impl Default for MoovIoAchConverters {
    fn default() -> Self {
        MoovIoAchConverters {
            moov_io_ach_string_zeros: HashMap::new(),
        }
    }
}

struct MoovIoAchAddenda98 {
    original_dfi: String,
    moov_io_ach_converters: Box<MoovIoAchConverters>,
}

