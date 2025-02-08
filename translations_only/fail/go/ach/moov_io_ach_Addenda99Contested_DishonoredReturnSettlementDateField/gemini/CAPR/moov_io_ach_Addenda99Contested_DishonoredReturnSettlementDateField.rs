
use std::collections::HashMap;
use std::fmt;
use std::ops::Deref;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct MoovIoAchAddenda99Contested {
    pub dishonored_return_settlement_date: String,
}

impl MoovIoAchAddenda99Contested {
    pub fn dishonored_return_settlement_date(&self) -> &str {
        &self.dishonored_return_settlement_date
    }
}

impl fmt::Display for MoovIoAchAddenda99Contested {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.dishonored_return_settlement_date)
    }
}

impl FromStr for MoovIoAchAddenda99Contested {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(MoovIoAchAddenda99Contested {
            dishonored_return_settlement_date: s.to_string(),
        })
    }
}

#[derive(Debug, Clone)]
pub struct MoovIoAchConverters {}

impl MoovIoAchConverters {
    pub fn string_field(&self, s: &str, max: usize) -> String {
        let ln = s.chars().count();
        if ln > max {
            return s[..max].to_string();
        }

        let m = max - ln;
        let pad = moov_io_ach_string_zeros(m);
        pad + s
    }
}

fn moov_io_ach_string_zeros(max: usize) -> String {
    let mut out = HashMap::new();
    for i in 0..max {
        out.insert(i, "0".repeat(i));
    }
    out[&max].clone()
}
