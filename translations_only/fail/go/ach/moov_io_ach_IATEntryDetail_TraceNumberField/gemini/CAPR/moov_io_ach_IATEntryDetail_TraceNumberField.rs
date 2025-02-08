
use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;
use std::string::ToString;

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

impl ToString for MoovIoAchIATEntryDetail {
    fn to_string(&self) -> String {
        format!(
            "TraceNumber: {}, Addenda10: {}, Addenda11: {}, Addenda12: {}, Addenda13: {}, Addenda14: {}, Addenda15: {}, Addenda16: {}, Addenda17: {:?}, Addenda18: {:?}, Addenda98: {:?}, Addenda99: {:?}, Category: {}",
            self.trace_number, self.addenda10, self.addenda11, self.addenda12, self.addenda13, self.addenda14, self.addenda15, self.addenda16, self.addenda17, self.addenda18, self.addenda98, self.addenda99, self.category
        )
    }
}

impl FromStr for MoovIoAchIATEntryDetail {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(',');
        let trace_number = parts.next().unwrap().to_string();
        let addenda10 = parts.next().unwrap().to_string();
        let addenda11 = parts.next().unwrap().to_string();
        let addenda12 = parts.next().unwrap().to_string();
        let addenda13 = parts.next().unwrap().to_string();
        let addenda14 = parts.next().unwrap().to_string();
        let addenda15 = parts.next().unwrap().to_string();
        let addenda16 = parts.next().unwrap().to_string();
        let addenda17 = parts.next().map(|s| s.to_string());
        let addenda18 = parts.next().map(|s| s.to_string());
        let addenda98 = parts.next().map(|s| s.to_string());
        let addenda99 = parts.next().map(|s| s.to_string());
        let category = parts.next().unwrap().to_string();

        Ok(MoovIoAchIATEntryDetail {
            trace_number,
            addenda10,
            addenda11,
            addenda12,
            addenda13,
            addenda14,
            addenda15,
            addenda16,
            addenda17,
            addenda18,
            addenda98,
            addenda99,
            category,
        })
    }
}

pub struct MoovIoAchConverters {}

impl MoovIoAchConverters {
    pub fn string_field(&self, s: &str, max: usize) -> String {
        let ln = s.chars().count();
        if ln > max {
            s[..max].to_string()
        } else {
            let m = max - ln;
            let pad = "0".repeat(m);
            format!("{}{}", pad, s)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_field() {
        let converters = MoovIoAchConverters {};
        assert_eq!(converters.string_field("12345", 5), "12345");
        assert_eq!(converters.string_field("1234567890", 5), "12345");
        assert_eq!(converters.string_field("12345", 10), "0000012345");
    }

    #[test]
    fn test_moov_io_ach_populate_map() {
        let out = moov_io_ach_populate_map(10, "0");
        assert_eq!(out.get(&0), Some(&""));
        assert_eq!(out.get(&1), Some(&"0"));
        assert_eq!(out.get(&9), Some(&"000000000"));
    }

    #[test]
    fn test_moov_io_ach_iate_entry_detail() {
        let iat_entry_detail = MoovIoAchIATEntryDetail {
            trace_number: "1234567890".to_string(),
            addenda10: "addenda10".to_string(),
            addenda11: "addenda11".to_string(),
            addenda12: "addenda12".to_string(),
            addenda13: "addenda13".to_string(),
            addenda14: "addenda14".to_string(),
            addenda15: "addenda15".to_string(),
            addenda16: "addenda16".to_string(),
            addenda17: None,
            addenda18: None,
            addenda98: None,
            addenda99: None,
            category: "category".to_string(),
        };

        assert_eq!(
            iat_entry_detail.to_string(),
            "TraceNumber: 1234567890, Addenda10: addenda10, Addenda11: addenda11, Addenda12: addenda12, Addenda13: addenda13, Addenda14: addenda14, Addenda15: addenda15, Addenda16: addenda16, Addenda17: None, Addenda18: None, Addenda98: None, Addenda99: None, Category: category"
        );

        let iat_entry_detail_from_str = MoovIoAchIATEntryDetail::from_str(
            "TraceNumber: 1234567890, Addenda10: addenda10, Addenda11: addenda11, Addenda12: addenda12, Addenda13: addenda13, Addenda14: addenda14, Addenda15: addenda15, Addenda16: addenda16, Addenda17: None, Addenda18: None, Addenda98: None, Addenda99: None, Category: category",
        )
        .unwrap();

        assert_eq!(iat_entry_detail, iat_entry_detail_from_str);
    }
}
