
use std::collections::HashMap;
use std::fmt;
use std::ops::Deref;
use std::str::FromStr;

#[derive(Debug)]
pub struct MoovIoAchIatBatchHeader {
    pub odfi_identification: String,
    pub batch_number: String,
}

impl MoovIoAchIatBatchHeader {
    pub fn odfi_identification_field(&self) -> &str {
        &self.odfi_identification
    }
}

impl FromStr for MoovIoAchIatBatchHeader {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.splitn(2, ',');
        let odfi_identification = parts.next().unwrap();
        let batch_number = parts.next().unwrap();

        Ok(MoovIoAchIatBatchHeader {
            odfi_identification: odfi_identification.to_string(),
            batch_number: batch_number.to_string(),
        })
    }
}

impl fmt::Display for MoovIoAchIatBatchHeader {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{},{}", self.odfi_identification, self.batch_number)
    }
}

#[derive(Debug)]
pub struct MoovIoAchConverters {}

impl MoovIoAchConverters {
    pub fn string_field(&self, s: &str, max: usize) -> String {
        let ln = s.chars().count();
        if ln > max {
            return s[..max].to_string();
        }

        let m = max - ln;
        let pad = MOOV_IO_ACH_STRING_ZEROS.get(&m).unwrap();
        format!("{}{}", pad, s)
    }
}

lazy_static::lazy_static! {
    static ref MOOV_IO_ACH_STRING_ZEROS: HashMap<usize, String> = {
        let mut out = HashMap::new();
        for i in 0..94 {
            out.insert(i, String::from("0".repeat(i)));
        }
        out
    };
}
