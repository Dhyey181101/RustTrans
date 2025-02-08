
use std::collections::HashMap;
use std::fmt::Display;
use std::ops::Deref;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct MoovIoAchAddenda98Refused {
    pub original_dfi: String,
    pub corrected_data: String,
    pub change_code: String,
    pub trace_sequence_number: String,
    pub trace_number: String,
    converters: MoovIoAchConverters,
}

impl MoovIoAchAddenda98Refused {
    pub fn original_dfi_field(&self) -> String {
        self.string_field(&self.original_dfi, 8)
    }

    fn string_field(&self, s: &str, max: usize) -> String {
        let ln = s.chars().count();
        if ln > max {
            s[..max].to_string()
        } else {
            let m = max - ln;
            let pad = moov_io_ach_string_zeros(m);
            format!("{}{}", pad, s)
        }
    }
}

fn moov_io_ach_string_zeros(max: usize) -> String {
    let mut out = String::new();
    for _ in 0..max {
        out.push('0');
    }
    out
}

fn moov_io_ach_populate_map(max: usize, zero: &str) -> HashMap<usize, String> {
    let mut out = HashMap::new();
    for i in 0..max {
        out.insert(i, zero.repeat(i));
    }
    out
}

#[derive(Debug, Clone)]
pub struct MoovIoAchConverters {}

impl MoovIoAchConverters {
    pub fn string_field(&self, s: &str, max: usize) -> String {
        let ln = s.chars().count();
        if ln > max {
            s[..max].to_string()
        } else {
            let m = max - ln;
            let pad = moov_io_ach_string_zeros(m);
            format!("{}{}", pad, s)
        }
    }
}

impl Deref for MoovIoAchAddenda98Refused {
    type Target = MoovIoAchConverters;

    fn deref(&self) -> &Self::Target {
        &self.converters
    }
}

impl FromStr for MoovIoAchAddenda98Refused {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(',');
        Ok(MoovIoAchAddenda98Refused {
            original_dfi: parts.next().unwrap().to_string(),
            corrected_data: parts.next().unwrap().to_string(),
            change_code: parts.next().unwrap().to_string(),
            trace_sequence_number: parts.next().unwrap().to_string(),
            trace_number: parts.next().unwrap().to_string(),
            converters: MoovIoAchConverters {},
        })
    }
}

impl Display for MoovIoAchAddenda98Refused {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{},{},{},{},{}",
            self.original_dfi, self.corrected_data, self.change_code,
            self.trace_sequence_number, self.trace_number
        )
    }
}
