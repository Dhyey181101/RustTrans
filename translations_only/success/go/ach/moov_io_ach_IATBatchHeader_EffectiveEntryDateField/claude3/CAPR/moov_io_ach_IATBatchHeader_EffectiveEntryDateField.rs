

use std::collections::HashMap;
use once_cell::sync::Lazy;

static MOOV_IO_ACH_STRING_ZEROS: Lazy<HashMap<usize, String>> = Lazy::new(|| {
    moov_io_ach_populate_map(94, String::from("0"))
});

struct MoovIoAchIatBatchHeader {
    effective_entry_date: String,
}

impl MoovIoAchIatBatchHeader {
    fn effective_entry_date_field(&self) -> String {
        string_field(&self.effective_entry_date, 6) // YYMMDD
    }
}

fn string_field(s: &str, max: u32) -> String {
    let ln = s.len() as u32;
    if ln > max {
        return s.chars().take(max as usize).collect();
    }

    let m = (max - ln) as usize;
    let pad = MOOV_IO_ACH_STRING_ZEROS.get(&m).unwrap();
    let mut out = pad.to_owned();
    out.push_str(s);
    out
}

fn moov_io_ach_populate_map(max: usize, zero: String) -> HashMap<usize, String> {
    let mut out = HashMap::with_capacity(max);
    for i in 0..max {
        out.insert(i, zero.repeat(i));
    }
    out
}

