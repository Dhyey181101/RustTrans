
use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct MoovIoAchAddenda98 {
    pub trace_number: String,
}

impl MoovIoAchAddenda98 {
    pub fn trace_number_field(&self) -> String {
        self.trace_number.clone()
    }
}

impl FromStr for MoovIoAchAddenda98 {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(MoovIoAchAddenda98 {
            trace_number: s.to_string(),
        })
    }
}

impl fmt::Display for MoovIoAchAddenda98 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.trace_number)
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
        let pad = "0".repeat(m);
        pad + s
    }
}

pub fn moov_io_ach_populate_map(max: usize, zero: &str) -> HashMap<usize, String> {
    let mut out = HashMap::new();
    for i in 0..max {
        out.insert(i, zero.repeat(i));
    }
    out
}
