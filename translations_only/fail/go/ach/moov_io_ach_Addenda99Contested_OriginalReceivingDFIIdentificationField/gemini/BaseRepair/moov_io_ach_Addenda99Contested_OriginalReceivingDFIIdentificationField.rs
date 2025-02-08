
use std::collections::HashMap;
use std::ops::Deref;
use std::str::FromStr;
use std::string::ToString;

#[derive(Debug, Clone, PartialEq)]
pub struct MoovIoAchAddenda99Contested {
    pub original_receiving_dfi_identification: String,
}

impl Deref for MoovIoAchAddenda99Contested {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.original_receiving_dfi_identification
    }
}

impl FromStr for MoovIoAchAddenda99Contested {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(MoovIoAchAddenda99Contested {
            original_receiving_dfi_identification: s.to_string(),
        })
    }
}

impl MoovIoAchAddenda99Contested {
    pub fn original_receiving_dfi_identification(&self) -> &str {
        &self.original_receiving_dfi_identification
    }
}

pub fn string_field(s: &str, max: usize) -> String {
    let ln = s.chars().count();
    if ln > max {
        s[..max].to_string()
    } else {
        let m = max - ln;
        let pad = moov_io_ach_string_zeros(m);
        format!("{}{}", pad, s)
    }
}

fn moov_io_ach_string_zeros(max: usize) -> String {
    let mut out = HashMap::new();
    for i in 0..max {
        out.insert(i, "0".repeat(i));
    }
    out[&max].clone()
}
