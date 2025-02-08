
use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct MoovIoAchEntryDetail {
    pub trace_number: String,
    pub addenda02: Option<MoovIoAchAddenda02>,
    pub addenda98: Option<MoovIoAchAddenda98>,
    pub addenda98_refused: Option<MoovIoAchAddenda98Refused>,
    pub addenda99: Option<MoovIoAchAddenda99>,
    pub addenda99_contested: Option<MoovIoAchAddenda99Contested>,
    pub addenda99_dishonored: Option<MoovIoAchAddenda99Dishonored>,
}

impl MoovIoAchEntryDetail {
    pub fn set_trace_number(&mut self, odfi_identification: &str, seq: i32) {
        let trace_number = format!(
            "{}{}",
            self.string_field(odfi_identification, 8),
            self.numeric_field(seq, 7)
        );
        self.trace_number = trace_number.clone();

        // Populate TraceNumber of addenda records that should match the Entry's trace number
        if let Some(addenda02) = &mut self.addenda02 {
            addenda02.trace_number = trace_number.clone();
        }
        if let Some(addenda98) = &mut self.addenda98 {
            addenda98.trace_number = trace_number.clone();
        }
        if let Some(addenda98_refused) = &mut self.addenda98_refused {
            addenda98_refused.trace_number = trace_number.clone();
        }
        if let Some(addenda99) = &mut self.addenda99 {
            addenda99.trace_number = trace_number.clone();
        }
        if let Some(addenda99_contested) = &mut self.addenda99_contested {
            addenda99_contested.trace_number = trace_number.clone();
        }
        if let Some(addenda99_dishonored) = &mut self.addenda99_dishonored {
            addenda99_dishonored.trace_number = trace_number.clone();
        }
    }

    fn string_field(&self, s: &str, max: u32) -> String {
        let ln = s.chars().count() as u32;
        if ln > max {
            return s[..max as usize].to_string();
        }

        // Pad with preallocated string
        let m = max - ln;
        let pad = MOOV_IO_ACH_STRING_ZEROS[m as usize].clone();
        format!("{}{}", pad, s)
    }

    fn numeric_field(&self, n: i32, max: u32) -> String {
        let s = n.to_string();
        let l = s.len() as u32;
        if l > max {
            return s[(l - max) as usize..].to_string();
        } else {
            let m = max - l;
            let pad = MOOV_IO_ACH_STRING_ZEROS[m as usize].clone();
            format!("{}{}", pad, s)
        }
    }
}

#[derive(Debug, Clone)]
pub struct MoovIoAchAddenda02 {
    pub trace_number: String,
}

#[derive(Debug, Clone)]
pub struct MoovIoAchAddenda98 {
    pub trace_number: String,
}

#[derive(Debug, Clone)]
pub struct MoovIoAchAddenda98Refused {
    pub trace_number: String,
}

#[derive(Debug, Clone)]
pub struct MoovIoAchAddenda99 {
    pub trace_number: String,
}

#[derive(Debug, Clone)]
pub struct MoovIoAchAddenda99Contested {
    pub trace_number: String,
}

#[derive(Debug, Clone)]
pub struct MoovIoAchAddenda99Dishonored {
    pub trace_number: String,
}

const MOOV_IO_ACH_STRING_ZEROS: [&str; 94] = ["0"; 94];

