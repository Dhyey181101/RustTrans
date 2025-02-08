

use std::collections::HashMap;
use std::str;
use once_cell::sync::Lazy;

static MOOV_IO_ACH_STRING_ZEROS: Lazy<HashMap<usize, String>> = Lazy::new(|| moov_io_ach_populate_map(94, "0".to_string()));

struct MoovIoAchAddenda99Contested {
    contested_return_code: String,
    original_entry_trace_number: String,
    date_original_entry_returned: String,
    original_receiving_dfi_identification: String,
    original_settlement_date: String,
    return_trace_number: String,
    return_settlement_date: String,
    return_reason_code: String,
    dishonored_return_trace_number: String,
    dishonored_return_settlement_date: String,
    dishonored_return_reason_code: String,
    trace_number: String,
    converters: Box<MoovIoAchConverters>,
}

impl MoovIoAchAddenda99Contested {
    fn original_entry_trace_number_field(&self) -> String {
        self.converters.string_field(self.original_entry_trace_number.clone(), 15)
    }
}

struct MoovIoAchConverters;

impl MoovIoAchConverters {
    fn string_field(&self, s: String, max: u32) -> String {
        let ln = s.chars().count() as u32;
        if ln > max {
            return s.chars().take(max as usize).collect();
        }

        let m = (max - ln) as usize;
        let binding = "".to_string();
        let pad = MOOV_IO_ACH_STRING_ZEROS.get(&m).unwrap_or(&binding);
        pad.to_string() + &s
    }
}

fn moov_io_ach_populate_map(max: usize, zero: String) -> HashMap<usize, String> {
    let mut out = HashMap::with_capacity(max);
    for i in 0..max {
        out.insert(i, zero.repeat(i));
    }
    out
}

