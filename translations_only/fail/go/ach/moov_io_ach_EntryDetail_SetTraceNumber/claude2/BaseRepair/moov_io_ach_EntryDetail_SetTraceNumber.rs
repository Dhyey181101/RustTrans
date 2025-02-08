
use std::collections::HashMap;
use std::str;
use once_cell::sync::Lazy;

static MOOV_IO_ACH_STRING_ZEROS: Lazy<HashMap<i32, String>> = Lazy::new(|| HashMap::new());

struct MoovIoAchEntryDetail {
    trace_number: String,
    addenda02: Option<Box<MoovIoAchAddenda02>>,
    addenda98: Option<Box<MoovIoAchAddenda98>>,
    addenda98_refused: Option<Box<MoovIoAchAddenda98Refused>>,
    addenda99: Option<Box<MoovIoAchAddenda99>>,
    addenda99_contested: Option<Box<MoovIoAchAddenda99Contested>>,
    addenda99_dishonored: Option<Box<MoovIoAchAddenda99Dishonored>>,
}

impl MoovIoAchEntryDetail {
    fn set_trace_number(&mut self, odfi_identification: &str, seq: i32) {
        let trace_number = self.string_field(odfi_identification, 8) + &self.numeric_field(seq, 7);
        self.trace_number = trace_number.clone();

        if let Some(ref mut addenda02) = self.addenda02 {
            addenda02.trace_number = trace_number.clone();
        }
        if let Some(ref mut addenda98) = self.addenda98 {
            addenda98.trace_number = trace_number.clone();
        }
        if let Some(ref mut addenda98_refused) = self.addenda98_refused {
            addenda98_refused.trace_number = trace_number.clone();
        }
        if let Some(ref mut addenda99) = self.addenda99 {
            addenda99.trace_number = trace_number.clone();
        }
        if let Some(ref mut addenda99_contested) = self.addenda99_contested {
            addenda99_contested.trace_number = trace_number.clone();
        }
        if let Some(ref mut addenda99_dishonored) = self.addenda99_dishonored {
            addenda99_dishonored.trace_number = trace_number.clone();
        }
    }

    fn string_field(&self, s: &str, max: u32) -> String {
        let ln = s.len() as u32;
        if ln > max {
            s[..max as usize].to_string()
        } else {
            let m = (max - ln) as usize;
            "0".repeat(m) + s
        }
    }

    fn numeric_field(&self, n: i32, max: u32) -> String {
        let s = n.to_string();
        let l = s.len() as u32;
        if l > max {
            s[(l - max) as usize..].to_string()
        } else {
            let m = (max - l) as usize;
            "0".repeat(m) + &s
        }
    }
}

struct MoovIoAchAddenda02 {
    trace_number: String,
}

struct MoovIoAchAddenda98 {
    trace_number: String,  
}

struct MoovIoAchAddenda98Refused {
    trace_number: String,
}

struct MoovIoAchAddenda99 {
    trace_number: String,
}

struct MoovIoAchAddenda99Contested {
    trace_number: String,
}

struct MoovIoAchAddenda99Dishonored {
    trace_number: String, 
}

