
use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;

#[derive(Debug)]
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
            MoovIoAchConverters {}.string_field(odfi_identification, 8),
            MoovIoAchConverters {}.numeric_field(seq, 7)
        );
    }
}

impl fmt::Display for MoovIoAchIATEntryDetail {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "MoovIoAchIATEntryDetail {{ trace_number: {}, addenda10: {}, addenda11: {}, addenda12: {}, addenda13: {}, addenda14: {}, addenda15: {}, addenda16: {}, addenda17: {:?}, addenda18: {:?}, addenda98: {:?}, addenda99: {:?}, category: {} }}",
            self.trace_number, self.addenda10, self.addenda11, self.addenda12, self.addenda13, self.addenda14, self.addenda15, self.addenda16, self.addenda17, self.addenda18, self.addenda98, self.addenda99, self.category
        )
    }
}

pub struct MoovIoAchConverters {}

impl MoovIoAchConverters {
    pub fn string_field(&self, s: &str, max: u32) -> String {
        let ln = s.chars().count() as u32;
        if ln > max {
            return s[..max as usize].to_string();
        }

        let m = max - ln;
        let pad = moov_io_ach_string_zeros(m as usize);
        format!("{}{}", pad, s)
    }

    pub fn numeric_field(&self, n: i32, max: u32) -> String {
        let s = n.to_string();
        let l = s.chars().count() as u32;
        if l > max {
            return s[(l - max) as usize..].to_string();
        } else {
            let m = max - l;
            let pad = moov_io_ach_string_zeros(m as usize);
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
