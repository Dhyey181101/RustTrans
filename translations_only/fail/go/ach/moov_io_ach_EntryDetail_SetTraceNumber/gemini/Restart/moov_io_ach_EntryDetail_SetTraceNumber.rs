
use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;
use std::string::ToString;

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
        let pad = get_moov_io_ach_string_zeros(m as i32);
        format!("{}{}", pad, s)
    }

    fn numeric_field(&self, n: i32, max: u32) -> String {
        let s = n.to_string();
        let l = s.len() as u32;
        if l > max {
            return s[(l - max) as usize..].to_string();
        } else {
            let m = max - l;
            let pad = get_moov_io_ach_string_zeros(m as i32);
            format!("{}{}", pad, s)
        }
    }
}

impl fmt::Display for MoovIoAchEntryDetail {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "MoovIoAchEntryDetail {{ trace_number: {}, addenda02: {:?}, addenda98: {:?}, addenda98_refused: {:?}, addenda99: {:?}, addenda99_contested: {:?}, addenda99_dishonored: {:?} }}", self.trace_number, self.addenda02, self.addenda98, self.addenda98_refused, self.addenda99, self.addenda99_contested, self.addenda99_dishonored)
    }
}

#[derive(Debug, Clone)]
pub struct MoovIoAchAddenda02 {
    pub trace_number: String,
}

impl fmt::Display for MoovIoAchAddenda02 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "MoovIoAchAddenda02 {{ trace_number: {} }}", self.trace_number)
    }
}

#[derive(Debug, Clone)]
pub struct MoovIoAchAddenda98 {
    pub trace_number: String,
}

impl fmt::Display for MoovIoAchAddenda98 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "MoovIoAchAddenda98 {{ trace_number: {} }}", self.trace_number)
    }
}

#[derive(Debug, Clone)]
pub struct MoovIoAchAddenda98Refused {
    pub trace_number: String,
}

impl fmt::Display for MoovIoAchAddenda98Refused {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "MoovIoAchAddenda98Refused {{ trace_number: {} }}", self.trace_number)
    }
}

#[derive(Debug, Clone)]
pub struct MoovIoAchAddenda99 {
    pub trace_number: String,
}

impl fmt::Display for MoovIoAchAddenda99 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "MoovIoAchAddenda99 {{ trace_number: {} }}", self.trace_number)
    }
}

#[derive(Debug, Clone)]
pub struct MoovIoAchAddenda99Contested {
    pub trace_number: String,
}

impl fmt::Display for MoovIoAchAddenda99Contested {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "MoovIoAchAddenda99Contested {{ trace_number: {} }}", self.trace_number)
    }
}

#[derive(Debug, Clone)]
pub struct MoovIoAchAddenda99Dishonored {
    pub trace_number: String,
}

impl fmt::Display for MoovIoAchAddenda99Dishonored {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "MoovIoAchAddenda99Dishonored {{ trace_number: {} }}", self.trace_number)
    }
}

pub fn get_moov_io_ach_string_zeros(i: i32) -> String {
    let mut s = String::new();
    for _ in 0..i {
        s.push('0');
    }
    s
}
