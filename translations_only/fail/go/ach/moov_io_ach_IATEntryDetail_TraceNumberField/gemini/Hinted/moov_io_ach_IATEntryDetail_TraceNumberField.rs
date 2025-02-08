
use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct MoovIoAchIATEntryDetail {
    pub trace_number: String,
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
    pub category: String,
}

impl MoovIoAchIATEntryDetail {
    pub fn trace_number_field(&self) -> String {
        self.trace_number.clone()
    }
}

impl FromStr for MoovIoAchIATEntryDetail {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(',');
        Ok(MoovIoAchIATEntryDetail {
            trace_number: parts.next().unwrap().to_string(),
            addenda10: parts.next().unwrap().to_string(),
            addenda11: parts.next().unwrap().to_string(),
            addenda12: parts.next().unwrap().to_string(),
            addenda13: parts.next().unwrap().to_string(),
            addenda14: parts.next().unwrap().to_string(),
            addenda15: parts.next().unwrap().to_string(),
            addenda16: parts.next().unwrap().to_string(),
            addenda17: parts.next().map(|s| s.to_string()),
            addenda18: parts.next().map(|s| s.to_string()),
            addenda98: parts.next().map(|s| s.to_string()),
            addenda99: parts.next().map(|s| s.to_string()),
            category: parts.next().unwrap().to_string(),
        })
    }
}

impl fmt::Display for MoovIoAchIATEntryDetail {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{},{},{},{},{},{},{},{},{},{},{},{},{}",
            self.trace_number, self.addenda10, self.addenda11, self.addenda12, self.addenda13,
            self.addenda14, self.addenda15, self.addenda16, self.addenda17.as_ref().unwrap_or(&"".to_string()),
            self.addenda18.as_ref().unwrap_or(&"".to_string()), self.addenda98.as_ref().unwrap_or(&"".to_string()),
            self.addenda99.as_ref().unwrap_or(&"".to_string()), self.category
        )
    }
}

pub fn moov_io_ach_populate_map(max: usize, zero: &str) -> HashMap<usize, String> {
    let mut out = HashMap::new();
    for i in 0..max {
        let pad = zero.repeat(i);
        let mut pad_clone = pad.clone();
        out.insert(i, pad_clone);
    }
    out
}

pub struct MoovIoAchConverters {}

impl MoovIoAchConverters {
    pub fn string_field(&self, s: &str, max: usize) -> String {
        let ln = s.chars().count();
        if ln > max {
            s[..max].to_string()
        } else {
            let m = max - ln;
            let pad = moov_io_ach_populate_map(m, "0").get(&m).unwrap().clone();
            format!("{}{}", pad, s)
        }
    }
}
