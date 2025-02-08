
use std::collections::HashMap;
use std::str;
use once_cell::sync::Lazy;

static MOOV_IO_ACH_STRING_ZEROS: Lazy<HashMap<usize, String>> = Lazy::new(|| moov_io_ach_populate_map(94, "0"));

fn moov_io_ach_populate_map(max: usize, zero: &str) -> HashMap<usize, String> {
    let mut out = HashMap::with_capacity(max);
    for i in 0..max {
        out.insert(i, zero.repeat(i));
    }
    out
}

impl MoovIoAchEntryDetail {
    fn set_trace_number(&mut self, odfi_identification: &str, seq: usize) {
        let trace_number = self.string_field(odfi_identification, 8) + &self.numeric_field(seq as i64, 7);
        self.trace_number = trace_number.clone();

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
            addenda99_dishonored.trace_number = trace_number;
        }
    }

    fn string_field(&self, s: &str, max: u32) -> String {
        let ln = s.chars().count() as u32;
        if ln > max {
            return s.chars().take(max as usize).collect();
        }

        let m = (max - ln) as usize;
        if let Some(pad) = MOOV_IO_ACH_STRING_ZEROS.get(&m) {
            pad.clone() + s
        } else {
            "0".repeat(m) + s
        }
    }

    fn numeric_field(&self, n: i64, max: u32) -> String {
        let s = n.to_string();
        let l = s.len() as u32;
        if l > max {
            return s.chars().skip((l - max) as usize).collect();
        } else {
            let m = (max - l) as usize;
            if let Some(pad) = MOOV_IO_ACH_STRING_ZEROS.get(&m) {
                pad.clone() + &s
            } else {
                "0".repeat(m) + &s
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
