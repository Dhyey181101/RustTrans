
use std::collections::HashMap;
use std::str::FromStr;
use std::string::ToString;

#[derive(Debug, Clone)]
pub struct Addenda99Dishonored {
    pub original_receiving_dfi_identification: String,
}

impl Addenda99Dishonored {
    pub fn original_receiving_dfi_identification(&self) -> &str {
        &self.original_receiving_dfi_identification
    }
}

pub struct Converters {}

impl Converters {
    pub fn string_field(&self, s: &str, max: usize) -> String {
        let ln = s.chars().count();
        if ln > max {
            s[..max].to_string()
        } else {
            let m = max - ln;
            let pad = moov_io_ach_populate_map(m, "0");
            pad + s
        }
    }
}

fn moov_io_ach_populate_map(max: usize, zero: &str) -> String {
    let mut out = HashMap::new();
    for i in 0..max {
        out.insert(i, zero.repeat(i));
    }
    out[&max].clone()
}
