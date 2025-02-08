
use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;
use std::string::ToString;

#[derive(Debug)]
pub struct MoovIoAchAddenda98Refused {
    pub trace_sequence_number: String,
}

impl MoovIoAchAddenda98Refused {
    pub fn trace_sequence_number_field(&self) -> String {
        self.trace_sequence_number.clone()
    }
}

impl ToString for MoovIoAchAddenda98Refused {
    fn to_string(&self) -> String {
        self.trace_sequence_number.clone()
    }
}

impl FromStr for MoovIoAchAddenda98Refused {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(MoovIoAchAddenda98Refused {
            trace_sequence_number: s.to_string(),
        })
    }
}

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
