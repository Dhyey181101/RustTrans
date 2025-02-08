
use std::collections::HashMap;
use std::fmt::Write;
use std::ops::Deref;
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
    pub fn set_trace_number(&mut self, odfi_identification: &str, seq: i32) {
        self.trace_number = format!(
            "{}{}",
            self.string_field(odfi_identification, 8),
            self.numeric_field(seq, 7)
        );
    }

    fn string_field(&self, s: &str, max: usize) -> String {
        let ln = s.chars().count();
        if ln > max {
            return s[..max].to_string();
        }

        let mut pad = String::new();
        let m = max - ln;
        for _ in 0..m {
            pad.push('0');
        }

        format!("{}{}", pad, s)
    }

    fn numeric_field(&self, n: i32, max: usize) -> String {
        let s = n.to_string();
        let l = s.chars().count();
        if l > max {
            return s[l - max..].to_string();
        } else {
            let mut pad = String::new();
            let m = max - l;
            for _ in 0..m {
                pad.push('0');
            }

            format!("{}{}", pad, s)
        }
    }
}

pub struct MoovIoAchConverters {}

impl MoovIoAchConverters {
    pub fn string_field(&self, s: &str, max: usize) -> String {
        let ln = s.chars().count();
        if ln > max {
            return s[..max].to_string();
        }

        let mut pad = String::new();
        let m = max - ln;
        for _ in 0..m {
            pad.push('0');
        }

        format!("{}{}", pad, s)
    }

    pub fn numeric_field(&self, n: i32, max: usize) -> String {
        let s = n.to_string();
        let l = s.chars().count();
        if l > max {
            return s[l - max..].to_string();
        } else {
            let mut pad = String::new();
            let m = max - l;
            for _ in 0..m {
                pad.push('0');
            }

            format!("{}{}", pad, s)
        }
    }
}

pub fn moov_io_ach_populate_map(max: usize, zero: &str) -> HashMap<usize, String> {
    let mut out = HashMap::new();
    for i in 0..max {
        let mut s = String::new();
        for _ in 0..i {
            s.push_str(zero);
        }
        out.insert(i, s);
    }
    out
}
