
use std::collections::HashMap;
use std::fmt;
use std::ops::Deref;
use std::str::FromStr;
use std::string::ToString;

#[derive(Clone, Debug)]
pub struct MoovIoAchEntryDetail {
    pub identification_number: String,
}

impl MoovIoAchEntryDetail {
    pub fn shr_document_reference_number_field(&self) -> String {
        self.string_field(&self.identification_number[4..15], 11)
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
    let mut out = HashMap::new();
    for i in 0..max {
        out.insert(i, "0".repeat(i));
    }
    out.get(&max).unwrap().to_string()
}

#[derive(Clone, Debug)]
pub struct MoovIoAchConverters {}

impl Deref for MoovIoAchConverters {
    type Target = MoovIoAchConverters;

    fn deref(&self) -> &Self::Target {
        self
    }
}

impl FromStr for MoovIoAchConverters {
    type Err = ();

    fn from_str(_s: &str) -> Result<Self, Self::Err> {
        Ok(MoovIoAchConverters {})
    }
}

impl fmt::Display for MoovIoAchConverters {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "MoovIoAchConverters")
    }
}
