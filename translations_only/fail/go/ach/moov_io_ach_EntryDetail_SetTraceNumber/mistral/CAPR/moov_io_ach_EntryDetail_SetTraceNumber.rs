

use std::collections::HashMap;
use std::fmt;
use std::str;

const ZEROS: &[u8] = b"0000000000000000000000000000000000000000000000000000000000000000";

struct MoovIoAchConverters;

impl MoovIoAchConverters {
    fn string_field(&self, s: &str, max: usize) -> String {
        let ln = s.len();
        if ln > max {
            s[..max].to_string()
        } else {
            let m = max - ln;
            let pad = &ZEROS[..m];
            if let Some(pad) = str::from_utf8(pad).ok() {
                format!("{}{}", pad, s)
            } else {
                format!("{:0<width$}", s, width = max)
            }
        }
    }

    fn numeric_field(&self, n: i32, max: usize) -> String {
        let s = n.to_string();
        if s.len() > max {
            s[..max].to_string()
        } else {
            let m = max - s.len();
            let pad = &ZEROS[..m];
            if let Some(pad) = str::from_utf8(pad).ok() {
                format!("{}{}", pad, s)
            } else {
                format!("{:0<width$}", s, width = max)
            }
        }
    }
}

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
        let moov_io_ach_converters = MoovIoAchConverters;
        let trace_number = moov_io_ach_converters.string_field(odfi_identification, 8);
        self.trace_number = format!("{}{:0<7}", trace_number, seq);

        if let Some(ref mut addenda02) = self.addenda02 {
            addenda02.set_trace_number(&self.trace_number);
        }
        if let Some(ref mut addenda98) = self.addenda98 {
            addenda98.set_trace_number(&self.trace_number);
        }
        if let Some(ref mut addenda98_refused) = self.addenda98_refused {
            addenda98_refused.set_trace_number(&self.trace_number);
        }
        if let Some(ref mut addenda99) = self.addenda99 {
            addenda99.set_trace_number(&self.trace_number);
        }
        if let Some(ref mut addenda99_contested) = self.addenda99_contested {
            addenda99_contested.set_trace_number(&self.trace_number);
        }
        if let Some(ref mut addenda99_dishonored) = self.addenda99_dishonored {
            addenda99_dishonored.set_trace_number(&self.trace_number);
        }
    }
}

struct MoovIoAchAddenda02 {
    trace_number: String,
}

impl MoovIoAchAddenda02 {
    fn set_trace_number(&mut self, trace_number: &str) {
        self.trace_number = trace_number.to_string();
    }
}

struct MoovIoAchAddenda98 {
    trace_number: String,
}

impl MoovIoAchAddenda98 {
    fn set_trace_number(&mut self, trace_number: &str) {
        self.trace_number = trace_number.to_string();
    }
}

struct MoovIoAchAddenda98Refused {
    trace_number: String,
}

impl MoovIoAchAddenda98Refused {
    fn set_trace_number(&mut self, trace_number: &str) {
        self.trace_number = trace_number.to_string();
    }
}

struct MoovIoAchAddenda99 {
    trace_number: String,
}

impl MoovIoAchAddenda99 {
    fn set_trace_number(&mut self, trace_number: &str) {
        self.trace_number = trace_number.to_string();
    }
}

struct MoovIoAchAddenda99Contested {
    trace_number: String,
}

impl MoovIoAchAddenda99Contested {
    fn set_trace_number(&mut self, trace_number: &str) {
        self.trace_number = trace_number.to_string();
    }
}

struct MoovIoAchAddenda99Dishonored {
    trace_number: String,
}

impl MoovIoAchAddenda99Dishonored {
    fn set_trace_number(&mut self, trace_number: &str) {
        self.trace_number = trace_number.to_string();
    }
}

fn main() {
    let mut entry_detail = Box::new(MoovIoAchEntryDetail {
        trace_number: String::new(),
        addenda02: None,
        addenda98: None,
        addenda98_refused: None,
        addenda99: None,
        addenda99_contested: None,
        addenda99_dishonored: None,
    });

    entry_detail.set_trace_number("12345678", 123456);
}

