
use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;
use std::string::ToString;

#[derive(Debug, Clone)]
pub struct MoovIoAchAddenda98Refused {
    pub trace_number: String,
}

impl MoovIoAchAddenda98Refused {
    pub fn trace_number_field(&self) -> String {
        self.trace_number.to_string()
    }
}

impl FromStr for MoovIoAchAddenda98Refused {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(MoovIoAchAddenda98Refused {
            trace_number: s.to_string(),
        })
    }
}

impl fmt::Display for MoovIoAchAddenda98Refused {
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
