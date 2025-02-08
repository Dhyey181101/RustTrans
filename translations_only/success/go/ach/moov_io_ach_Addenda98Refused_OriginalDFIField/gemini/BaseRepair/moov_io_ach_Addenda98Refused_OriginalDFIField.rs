
use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    static ref MOOV_IO_ACH_STRING_ZEROS: HashMap<usize, String> = (0..94)
        .map(|i| (i, String::from("0").repeat(i)))
        .collect();
}

pub struct MoovIoAchAddenda98Refused {
    pub original_dfi: String,
}

impl MoovIoAchAddenda98Refused {
    pub fn original_dfi_field(&self) -> String {
        string_field(&self.original_dfi, 8)
    }
}

pub struct MoovIoAchConverters {}

fn string_field(s: &str, max: usize) -> String {
    let ln = s.chars().count();
    if ln > max {
        s[..max].to_string()
    } else {
        let m = max - ln;
        if let Some(pad) = MOOV_IO_ACH_STRING_ZEROS.get(&m) {
            format!("{}{}", pad, s)
        } else {
            format!("{:0width$}", s, width = m)
        }
    }
}
