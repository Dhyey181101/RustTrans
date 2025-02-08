

use std::collections::HashMap;
use std::str;
use once_cell::sync::Lazy;

static ZEROS: Lazy<HashMap<usize, String>> = Lazy::new(|| populate_map(94, "0"));

struct Addenda99 {
    original_dfi: String,
}

impl Addenda99 {
    fn original_dfi_field(&self) -> String {
        let converters = Converters;
        converters.string_field(&self.original_dfi, 8)
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
        if let Some(pad) = ZEROS.get(&m) {
            return pad.to_owned() + s;
        }

        // slow path
        "0".repeat(m) + s
    }
}

fn populate_map(max: usize, zero: &str) -> HashMap<usize, String> {
    let mut out = HashMap::with_capacity(max);
    for i in 0..max {
        out.insert(i, zero.repeat(i));
    }
    out
}

