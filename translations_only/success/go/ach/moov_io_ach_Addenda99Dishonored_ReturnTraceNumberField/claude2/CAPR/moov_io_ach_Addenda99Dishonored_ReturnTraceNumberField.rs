
use std::collections::HashMap;
use once_cell::sync::Lazy;

static MOOV_IO_ACH_STRING_ZEROS: Lazy<HashMap<usize, String>> = Lazy::new(|| HashMap::new());

struct MoovIoAchAddenda99Dishonored {
    return_trace_number: String,
}

impl MoovIoAchAddenda99Dishonored {
    fn return_trace_number_field(&self) -> String {
        string_field(&self.return_trace_number, 15)
    }
}

fn string_field(s: &String, max: u32) -> String {
    let ln = s.len() as u32;
    if ln > max {
        s[..max as usize].to_string()
    } else {
        let m = (max - ln) as usize;
        let mut zeros = String::with_capacity(m);
        for _ in 0..m {
            zeros.push('0');
        }
        zeros + s
    }
}

