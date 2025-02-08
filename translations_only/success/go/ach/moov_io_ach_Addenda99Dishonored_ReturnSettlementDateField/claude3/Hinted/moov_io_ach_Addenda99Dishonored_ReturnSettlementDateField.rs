
use std::collections::HashMap;

static MOOV_IO_ACH_STRING_ZEROS: Lazy<HashMap<usize, String>> = Lazy::new(|| moov_io_ach_populate_map(94, "0".to_string()));

struct Addenda99Dishonored {
    return_settlement_date: String,
    converters: Converters,
}

impl Addenda99Dishonored {
    fn return_settlement_date_field(&self) -> String {
        self.converters.string_field(&self.return_settlement_date, 3)
    }
}

struct Converters;

impl Converters {
    fn string_field(&self, s: &str, max: u32) -> String {
        let ln = s.chars().count() as u32;
        if ln > max {
            return s.chars().take(max as usize).collect();
        }

        let m = (max - ln) as usize;
        if let Some(pad) = MOOV_IO_ACH_STRING_ZEROS.get(&m) {
            return pad.clone() + s;
        }

        // slow path
        "0".repeat(m) + s
    }
}

fn moov_io_ach_populate_map(max: usize, zero: String) -> HashMap<usize, String> {
    let mut out = HashMap::with_capacity(max);
    for i in 0..max {
        out.insert(i, zero.repeat(i));
    }
    out
}

use once_cell::sync::Lazy;
