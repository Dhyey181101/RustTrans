

use std::collections::HashMap;
use std::fmt;

#[derive(Default)]
struct MoovIoAchConverters;

impl MoovIoAchConverters {
    fn string_field(&self, s: &str, max: usize) -> String {
        let ln = s.len();
        if ln > max {
            return s[..max].to_string();
        }

        let m = max - ln;
        let pad = get_pad_string(m);
        pad + s
    }
}

fn get_pad_string(n: usize) -> String {
    let mut out = HashMap::new();
    for i in 0..=n {
        out.insert(i, "0".repeat(i));
    }
    out[&n].to_string()
}

#[derive(Default)]
struct MoovIoAchAddenda99Dishonored {
    original_receiving_dfi_identification: String,
    moov_io_ach_converters: MoovIoAchConverters,
}

impl MoovIoAchAddenda99Dishonored {
    fn original_receiving_dfi_identification_field(&self) -> String {
        self.moov_io_ach_converters.string_field(&self.original_receiving_dfi_identification, 8)
    }
}

impl fmt::Display for MoovIoAchAddenda99Dishonored {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "original_receiving_dfi_identification: {}",
            self.original_receiving_dfi_identification
        )
    }
}

