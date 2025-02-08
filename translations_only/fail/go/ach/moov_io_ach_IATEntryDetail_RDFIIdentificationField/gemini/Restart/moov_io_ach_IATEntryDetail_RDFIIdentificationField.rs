
use std::collections::HashMap;
use std::fmt;
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
    pub fn r_d_f_i_identification_field(&self) -> &str {
        &self.r_d_f_i_identification
    }
}

impl FromStr for MoovIoAchIATEntryDetail {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut addenda_10 = None;
        let mut addenda_11 = None;
        let mut addenda_12 = None;
        let mut addenda_13 = None;
        let mut addenda_14 = None;
        let mut addenda_15 = None;
        let mut addenda_16 = None;
        let mut addenda_17 = None;
        let mut addenda_18 = None;
        let mut addenda_98 = None;
        let mut addenda_99 = None;

        for line in s.lines() {
            if line.starts_with("10") {
                addenda_10 = Some(line.to_string());
            } else if line.starts_with("11") {
                addenda_11 = Some(line.to_string());
            } else if line.starts_with("12") {
                addenda_12 = Some(line.to_string());
            } else if line.starts_with("13") {
                addenda_13 = Some(line.to_string());
            } else if line.starts_with("14") {
                addenda_14 = Some(line.to_string());
            } else if line.starts_with("15") {
                addenda_15 = Some(line.to_string());
            } else if line.starts_with("16") {
                addenda_16 = Some(line.to_string());
            } else if line.starts_with("17") {
                addenda_17 = Some(line.to_string());
            } else if line.starts_with("18") {
                addenda_18 = Some(line.to_string());
            } else if line.starts_with("98") {
                addenda_98 = Some(line.to_string());
            } else if line.starts_with("99") {
                addenda_99 = Some(line.to_string());
            }
        }

        Ok(MoovIoAchIATEntryDetail {
            r_d_f_i_identification: String::new(),
            addenda_10: addenda_10.unwrap_or_default(),
            addenda_11: addenda_11.unwrap_or_default(),
            addenda_12: addenda_12.unwrap_or_default(),
            addenda_13: addenda_13.unwrap_or_default(),
            addenda_14: addenda_14.unwrap_or_default(),
            addenda_15: addenda_15.unwrap_or_default(),
            addenda_16: addenda_16.unwrap_or_default(),
            addenda_17: addenda_17,
            addenda_18: addenda_18,
            addenda_98: addenda_98,
            addenda_99: addenda_99,
        })
    }
}

impl fmt::Display for MoovIoAchIATEntryDetail {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "MoovIoAchIATEntryDetail {{ r_d_f_i_identification: {}, addenda_10: {}, addenda_11: {}, addenda_12: {}, addenda_13: {}, addenda_14: {}, addenda_15: {}, addenda_16: {}, addenda_17: {:?}, addenda_18: {:?}, addenda_98: {:?}, addenda_99: {:?} }}",
            self.r_d_f_i_identification, self.addenda_10, self.addenda_11, self.addenda_12, self.addenda_13, self.addenda_14, self.addenda_15, self.addenda_16, self.addenda_17, self.addenda_18, self.addenda_98, self.addenda_99
        )
    }
}

pub fn string_field(s: &str, max: usize) -> String {
    let ln = s.chars().count();
    if ln > max {
        return s[..max].to_string();
    }

    let m = max - ln;
    let pad = &MOOV_IO_ACH_STRING_ZEROS[&m];
    format!("{}{}", pad, s)
}

lazy_static::lazy_static! {
    static ref MOOV_IO_ACH_STRING_ZEROS: HashMap<usize, String> = {
        let mut out = HashMap::new();
        for i in 0..94 {
            out.insert(i, String::from_utf8(vec![b'0'; i]).unwrap());
        }
        out
    };
}
