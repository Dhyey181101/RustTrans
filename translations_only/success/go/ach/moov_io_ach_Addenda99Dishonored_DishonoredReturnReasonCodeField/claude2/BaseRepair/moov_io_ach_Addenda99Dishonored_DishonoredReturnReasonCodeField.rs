

use std::collections::HashMap;
use once_cell::sync::Lazy;

static MOOV_IO_ACH_STRINGZEROS: Lazy<HashMap<usize, String>> = Lazy::new(|| {
    populate_map(94, "0".to_string())
});

struct MoovIoAchAddenda99Dishonored {
    dishonored_return_reason_code: String,
}

impl MoovIoAchAddenda99Dishonored {
    fn dishonored_return_reason_code_field(&self, converters: &MoovIoAchConverters) -> String {
        converters.string_field(self.dishonored_return_reason_code.clone(), 3)
    }
}

struct MoovIoAchConverters;

impl MoovIoAchConverters {
    fn string_field(&self, s: String, max: u32) -> String {
        let ln = s.chars().count() as u32;
        if ln > max {
            s[..max as usize].to_string()
        } else {
            let m = max - ln;
            let pad = MOOV_IO_ACH_STRINGZEROS.get(&(m as usize)).unwrap();
            format!("{}{}", pad, s)
        }
    }
}

fn populate_map(max: usize, zero: String) -> HashMap<usize, String> {
    let mut map = HashMap::with_capacity(max);
    for i in 0..max {
        map.insert(i, zero.repeat(i));
    }
    map
}

