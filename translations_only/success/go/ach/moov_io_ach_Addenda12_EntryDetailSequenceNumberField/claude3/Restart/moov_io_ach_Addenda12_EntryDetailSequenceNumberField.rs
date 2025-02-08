

use std::collections::HashMap;
use once_cell::sync::Lazy;

static MOOV_IO_ACH_STRING_ZEROS: Lazy<HashMap<usize, String>> = Lazy::new(|| moov_io_ach_populate_map(94, "0".to_string()));

struct MoovIoAchAddenda12 {
    entry_detail_sequence_number: i32,
    converters: Box<MoovIoAchConverters>,
}

impl MoovIoAchAddenda12 {
    fn entry_detail_sequence_number_field(&self) -> String {
        self.converters.numeric_field(self.entry_detail_sequence_number, 7)
    }
}

struct MoovIoAchConverters {}

impl MoovIoAchConverters {
    fn numeric_field(&self, n: i32, max: u32) -> String {
        let s = n.to_string();
        let l = s.len();
        if l > max as usize {
            return s[(l - max as usize)..].to_string();
        } else {
            let m = (max as usize) - l;
            let pad = MOOV_IO_ACH_STRING_ZEROS.get(&m).unwrap_or(&"".to_string()).to_owned();
            return format!("{}{}", pad, s);
        }
    }
}

fn moov_io_ach_populate_map(max: usize, zero: String) -> HashMap<usize, String> {
    let mut out = HashMap::with_capacity(max);
    for i in 0..max {
        out.insert(i, zero.repeat(i));
    }
    out
}

