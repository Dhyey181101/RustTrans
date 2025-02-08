

use std::collections::HashMap;

struct MoovIoAchAddenda99Dishonored {
    dishonored_return_reason_code: String,
}

impl MoovIoAchAddenda99Dishonored {
    fn dishonored_return_reason_code_field(&self) -> String {
        MoovIoAchConverters::string_field(self.dishonored_return_reason_code.clone(), 3)
    }
}

struct MoovIoAchConverters;

impl MoovIoAchConverters {
    fn string_field(s: String, max: u32) -> String {
        let ln = s.len() as u32;
        if ln > max {
            s[..max as usize].to_string()
        } else {
            let m = max - ln;
            let pad = MOOV_IO_ACH_STRINGZEROS.get(&(m as usize)).unwrap().clone();
            format!("{}{}", pad, s)
        }
    }
}

static MOOV_IO_ACH_STRINGZEROS: once_cell::sync::Lazy<HashMap<usize, String>> = once_cell::sync::Lazy::new(|| {
    let mut map = HashMap::with_capacity(94);
    for i in 0..94 {
        map.insert(i, "0".repeat(i));
    }
    map
});

