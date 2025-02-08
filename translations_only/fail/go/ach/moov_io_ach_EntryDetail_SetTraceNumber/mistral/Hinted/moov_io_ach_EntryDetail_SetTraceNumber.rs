

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
            let pad = get_zeros(m);
            format!("{}{}", pad, s)
        }
    }

    fn numeric_field(&self, n: i32, max: usize) -> String {
        let s = n.to_string();
        let l = s.len();
        if l > max {
            s[l - max..].to_string()
        } else {
            let m = max - l;
            let pad = get_zeros(m);
            format!("{}{}", pad, s)
        }
    }
}

fn get_zeros(n: usize) -> String {
    if n <= 0 {
        String::from("")
    } else {
        unsafe { String::from_utf8_unchecked(ZEROS[..n].to_vec()) }
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
        let trace_number = format!("{}{:07}", trace_number, seq);
        self.trace_number = trace_number.clone();

        match &mut self.addenda02 {
            Some(addenda02) => addenda02.trace_number = trace_number.clone(),
            None => {}
        }
        match &mut self.addenda98 {
            Some(addenda98) => addenda98.trace_number = trace_number.clone(),
            None => {}
        }
        match &mut self.addenda98_refused {
            Some(addenda98_refused) => addenda98_refused.trace_number = trace_number.clone(),
            None => {}
        }
        match &mut self.addenda99 {
            Some(addenda99) => addenda99.trace_number = trace_number.clone(),
            None => {}
        }
        match &mut self.addenda99_contested {
            Some(addenda99_contested) => addenda99_contested.trace_number = trace_number.clone(),
            None => {}
        }
        match &mut self.addenda99_dishonored {
            Some(addenda99_dishonored) => addenda99_dishonored.trace_number = trace_number,
            None => {}
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

fn main() {
    let mut ed = Box::new(MoovIoAchEntryDetail {
        trace_number: String::from(""),
        addenda02: None,
        addenda98: None,
        addenda98_refused: None,
        addenda99: None,
        addenda99_contested: None,
        addenda99_dishonored: None,
    });

    let odfi_identification = "12345678";
    let seq = 123;
    ed.set_trace_number(odfi_identification, seq);
}

