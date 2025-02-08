
use std::collections::HashMap;
use std::fmt;
use std::ops::Deref;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct MoovIoAchIATEntryDetail {
    pub r_d_f_i_identification: String,
    pub addenda_10: String,
    pub addenda_11: String,
    pub addenda_12: String,
    pub addenda_13: String,
    pub addenda_14: String,
    pub addenda_15: String,
    pub addenda_16: String,
    pub addenda_17: Option<String>,
    pub addenda_18: Option<String>,
    pub addenda_98: Option<String>,
    pub addenda_99: Option<String>,
}

impl MoovIoAchIATEntryDetail {
    pub fn r_d_f_i_identification_field(&self) -> String {
        self.r_d_f_i_identification.clone()
    }
}

impl FromStr for MoovIoAchIATEntryDetail {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.splitn(18, '|');

        Ok(MoovIoAchIATEntryDetail {
            r_d_f_i_identification: parts.next().unwrap().to_string(),
            addenda_10: parts.next().unwrap().to_string(),
            addenda_11: parts.next().unwrap().to_string(),
            addenda_12: parts.next().unwrap().to_string(),
            addenda_13: parts.next().unwrap().to_string(),
            addenda_14: parts.next().unwrap().to_string(),
            addenda_15: parts.next().unwrap().to_string(),
            addenda_16: parts.next().unwrap().to_string(),
            addenda_17: parts.next().map(|s| s.to_string()),
            addenda_18: parts.next().map(|s| s.to_string()),
            addenda_98: parts.next().map(|s| s.to_string()),
            addenda_99: parts.next().map(|s| s.to_string()),
        })
    }
}

impl fmt::Display for MoovIoAchIATEntryDetail {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{:08}{:018}{:018}{:018}{:018}{:018}{:018}{:018}{:018}{:018}{:018}{:018}",
            self.r_d_f_i_identification,
            self.addenda_10,
            self.addenda_11,
            self.addenda_12,
            self.addenda_13,
            self.addenda_14,
            self.addenda_15,
            self.addenda_16,
            self.addenda_17.as_ref().unwrap_or(&"".to_string()),
            self.addenda_18.as_ref().unwrap_or(&"".to_string()),
            self.addenda_98.as_ref().unwrap_or(&"".to_string()),
            self.addenda_99.as_ref().unwrap_or(&"".to_string()),
        )
    }
}

pub struct MoovIoAchConverters {}

impl MoovIoAchConverters {
    pub fn string_field(&self, s: &str, max: usize) -> String {
        let ln = s.chars().count();
        if ln > max {
            s[..max].to_string()
        } else {
            let mut pad = String::new();
            for _ in 0..max - ln {
                pad.push('0');
            }
            pad + s
        }
    }
}

pub fn moov_io_ach_populate_map(max: usize, zero: &str) -> HashMap<usize, String> {
    let mut out = HashMap::new();
    for i in 0..max {
        out.insert(i, zero.repeat(i));
    }
    out
}
