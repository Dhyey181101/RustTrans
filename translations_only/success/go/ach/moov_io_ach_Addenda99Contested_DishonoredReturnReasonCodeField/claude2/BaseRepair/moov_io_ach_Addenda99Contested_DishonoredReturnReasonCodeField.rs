
use std::collections::HashMap;
use once_cell::sync::Lazy;

struct MoovIoAchAddenda99Contested {
    dishonored_return_reason_code: String,
}

impl MoovIoAchAddenda99Contested {
    fn dishonored_return_reason_code_field(&self) -> String {
        MoovIoAchConverters::string_field(self.dishonored_return_reason_code.clone(), 2)
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
            let pad = MOOV_IO_ACH_STRING_ZEROS.get(&(m as usize)).unwrap();
            format!("{}{}", pad, s)
        }
    }
}

static MOOV_IO_ACH_STRING_ZEROS: Lazy<HashMap<usize, String>> = Lazy::new(|| {
    let mut out = HashMap::with_capacity(94);
    for i in 0..94 {
        out.insert(i, "0".repeat(i));
    }
    out
});

