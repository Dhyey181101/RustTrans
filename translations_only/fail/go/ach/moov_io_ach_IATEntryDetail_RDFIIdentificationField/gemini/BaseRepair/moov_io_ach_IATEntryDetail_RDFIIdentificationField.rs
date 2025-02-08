
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;

#[derive(Debug)]
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
        let mut fields = s.split(',');
        Ok(MoovIoAchIATEntryDetail {
            r_d_f_i_identification: fields.next().unwrap().to_string(),
            addenda_10: fields.next().unwrap().to_string(),
            addenda_11: fields.next().unwrap().to_string(),
            addenda_12: fields.next().unwrap().to_string(),
            addenda_13: fields.next().unwrap().to_string(),
            addenda_14: fields.next().unwrap().to_string(),
            addenda_15: fields.next().unwrap().to_string(),
            addenda_16: fields.next().unwrap().to_string(),
            addenda_17: fields.next().map(|s| s.to_string()),
            addenda_18: fields.next().map(|s| s.to_string()),
            addenda_98: fields.next().map(|s| s.to_string()),
            addenda_99: fields.next().map(|s| s.to_string()),
        })
    }
}

impl fmt::Display for MoovIoAchIATEntryDetail {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{},{},{},{},{},{},{},{},{},{},{},{}",
            self.r_d_f_i_identification, self.addenda_10, self.addenda_11, self.addenda_12,
            self.addenda_13, self.addenda_14, self.addenda_15, self.addenda_16,
            self.addenda_17.as_ref().unwrap_or(&"".to_string()),
            self.addenda_18.as_ref().unwrap_or(&"".to_string()),
            self.addenda_98.as_ref().unwrap_or(&"".to_string()),
            self.addenda_99.as_ref().unwrap_or(&"".to_string()),
        )
    }
}

pub fn string_field(s: &str, max: usize) -> String {
    let ln = s.chars().count();
    if ln > max {
        s[..max].to_string()
    } else {
        let m = max - ln;
        let pad = moov_io_ach_string_zeros.get(&m).unwrap();
        format!("{}{}", pad, s)
    }
}

lazy_static! {
    static ref moov_io_ach_string_zeros: HashMap<usize, String> = {
        let mut out = HashMap::new();
        for i in 0..94 {
            out.insert(i, "0".repeat(i));
        }
        out
    };
}
