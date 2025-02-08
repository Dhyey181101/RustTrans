
use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct MoovIoAchIATEntryDetail {
    pub r_d_f_i_identification: String,
    pub addenda10: String,
    pub addenda11: String,
    pub addenda12: String,
    pub addenda13: String,
    pub addenda14: String,
    pub addenda15: String,
    pub addenda16: String,
    pub addenda17: Option<String>,
    pub addenda18: Option<String>,
    pub addenda98: Option<String>,
    pub addenda99: Option<String>,
}

impl MoovIoAchIATEntryDetail {
    pub fn r_d_f_i_identification_field(&self) -> String {
        self.r_d_f_i_identification.clone()
    }
}

impl FromStr for MoovIoAchIATEntryDetail {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut addenda10 = None;
        let mut addenda11 = None;
        let mut addenda12 = None;
        let mut addenda13 = None;
        let mut addenda14 = None;
        let mut addenda15 = None;
        let mut addenda16 = None;
        let mut addenda17 = None;
        let mut addenda18 = None;
        let mut addenda98 = None;
        let mut addenda99 = None;

        for line in s.lines() {
            if line.starts_with("10") {
                addenda10 = Some(line.to_string());
            } else if line.starts_with("11") {
                addenda11 = Some(line.to_string());
            } else if line.starts_with("12") {
                addenda12 = Some(line.to_string());
            } else if line.starts_with("13") {
                addenda13 = Some(line.to_string());
            } else if line.starts_with("14") {
                addenda14 = Some(line.to_string());
            } else if line.starts_with("15") {
                addenda15 = Some(line.to_string());
            } else if line.starts_with("16") {
                addenda16 = Some(line.to_string());
            } else if line.starts_with("17") {
                addenda17 = Some(line.to_string());
            } else if line.starts_with("18") {
                addenda18 = Some(line.to_string());
            } else if line.starts_with("98") {
                addenda98 = Some(line.to_string());
            } else if line.starts_with("99") {
                addenda99 = Some(line.to_string());
            }
        }

        Ok(MoovIoAchIATEntryDetail {
            r_d_f_i_identification: String::from("RDFIIdentification"),
            addenda10: addenda10.unwrap_or_default(),
            addenda11: addenda11.unwrap_or_default(),
            addenda12: addenda12.unwrap_or_default(),
            addenda13: addenda13.unwrap_or_default(),
            addenda14: addenda14.unwrap_or_default(),
            addenda15: addenda15.unwrap_or_default(),
            addenda16: addenda16.unwrap_or_default(),
            addenda17: addenda17,
            addenda18: addenda18,
            addenda98: addenda98,
            addenda99: addenda99,
        })
    }
}

impl fmt::Display for MoovIoAchIATEntryDetail {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "MoovIoAchIATEntryDetail {{ r_d_f_i_identification: {}, addenda10: {}, addenda11: {}, addenda12: {}, addenda13: {}, addenda14: {}, addenda15: {}, addenda16: {}, addenda17: {:?}, addenda18: {:?}, addenda98: {:?}, addenda99: {:?} }}",
            self.r_d_f_i_identification, self.addenda10, self.addenda11, self.addenda12, self.addenda13, self.addenda14, self.addenda15, self.addenda16, self.addenda17, self.addenda18, self.addenda98, self.addenda99
        )
    }
}

#[derive(Debug, Clone)]
pub struct MoovIoAchConverters {}

impl MoovIoAchConverters {
    pub fn string_field(&self, s: &str, max: u32) -> String {
        let ln = s.chars().count() as u32;
        if ln > max {
            return s[..max as usize].to_string();
        }

        let m = max - ln;
        let pad = "0".repeat(m as usize);
        format!("{}{}", pad, s)
    }
}

pub fn moov_io_ach_populate_map(max: usize, zero: &str) -> HashMap<usize, String> {
    let mut out = HashMap::new();
    for i in 0..max {
        out.insert(i, zero.repeat(i));
    }
    out
}
