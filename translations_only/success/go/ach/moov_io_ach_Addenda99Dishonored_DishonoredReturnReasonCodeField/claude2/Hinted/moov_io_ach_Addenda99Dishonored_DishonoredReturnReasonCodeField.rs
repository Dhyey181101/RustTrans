
use std::collections::HashMap;

static MOOV_IO_ACH_STRINGZEROS: once_cell::sync::Lazy<HashMap<usize, String>> = once_cell::sync::Lazy::new(|| moov_io_ach_populate_map(94, "0".to_string()));

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
        let ln = s.len();
        if ln > max as usize {
            s[..max as usize].to_string()
        } else {
            let m = (max - ln as u32) as usize;
            let pad = MOOV_IO_ACH_STRINGZEROS.get(&m).cloned().unwrap_or_else(|| "0".repeat(m));
            format!("{}{}", pad, s)
        }
    }
}

fn moov_io_ach_populate_map(max: usize, zero: String) -> HashMap<usize, String> {
    let mut out = HashMap::with_capacity(max);
    for i in 0..max {
        out.insert(i, "0".repeat(i));
    }
    out
}

