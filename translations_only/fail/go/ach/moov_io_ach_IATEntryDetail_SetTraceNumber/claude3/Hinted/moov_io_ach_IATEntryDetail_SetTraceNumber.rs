

use std::collections::HashMap;
use std::str;
use once_cell::sync::Lazy;

static MOOV_IO_ACH_STRING_ZEROS: Lazy<HashMap<usize, String>> = Lazy::new(|| moov_io_ach_populate_map(94, "0".to_string()));

struct MoovIoAchIatEntryDetail {
    trace_number: String,
    moov_io_ach_converters: Box<MoovIoAchConverters>,
}

struct MoovIoAchConverters;

impl MoovIoAchIatEntryDetail {
    fn set_trace_number(&mut self, odfi_identification: &str, seq: usize) {
        self.trace_number = self.moov_io_ach_converters.string_field(odfi_identification, 8)
            + &self.moov_io_ach_converters.numeric_field(seq as i32, 7);
    }
}

impl MoovIoAchConverters {
    fn string_field(&self, s: &str, max: u32) -> String {
        let ln = s.chars().count();
        if ln > max as usize {
            return s.chars().take(max as usize).collect();
        }

        let m = (max as usize) - ln;
        let pad = MOOV_IO_ACH_STRING_ZEROS.get(&m).map_or(String::new(), |s| s.to_string());
        pad + s
    }

    fn numeric_field(&self, n: i32, max: u32) -> String {
        let s = n.to_string();
        let l = s.len();
        if l > max as usize {
            return s.chars().skip(l - max as usize).collect();
        } else {
            let m = (max as usize) - l;
            let pad = MOOV_IO_ACH_STRING_ZEROS.get(&m).map_or(String::new(), |s| s.to_string());
            pad + &s
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


