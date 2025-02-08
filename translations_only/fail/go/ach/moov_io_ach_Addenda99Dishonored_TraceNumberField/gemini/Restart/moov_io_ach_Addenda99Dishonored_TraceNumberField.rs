
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug)]
pub struct MoovIoAchAddenda99Dishonored {
    pub trace_number: String,
}

impl MoovIoAchAddenda99Dishonored {
    pub fn trace_number_field(&self) -> String {
        self.string_field(&self.trace_number, 15)
    }

    fn string_field(&self, s: &str, max: usize) -> String {
        let ln = s.chars().count();
        if ln > max {
            s[..max].to_string()
        } else {
            let m = max - ln;
            let pad = MOOV_IO_ACH_STRING_ZEROS.get(&m).unwrap_or_else(|| {
                panic!("Failed to get pad for m: {}", m)
            });
            format!("{}{}", pad, s)
        }
    }
}

lazy_static! {
    static ref MOOV_IO_ACH_STRING_ZEROS: HashMap<usize, String> = {
        let mut out = HashMap::new();
        for i in 0..94 {
            out.insert(i, String::from_str("0").unwrap().repeat(i));
        }
        out
    };
}
