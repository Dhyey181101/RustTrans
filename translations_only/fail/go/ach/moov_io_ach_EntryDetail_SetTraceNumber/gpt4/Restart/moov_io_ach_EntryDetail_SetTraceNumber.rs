
use std::collections::HashMap;
use std::str;

lazy_static! {
    static ref MOOV_IO_ACH_STRING_ZEROS: HashMap<usize, String> = moov_io_ach_populate_map(94, "0");
}

fn moov_io_ach_populate_map(max: usize, zero: &str) -> HashMap<usize, String> {
    let mut out = HashMap::with_capacity(max);
    for i in 0..max {
        out.insert(i, zero.repeat(i));
    }
    out
}

struct MoovIoAchEntryDetail {
    trace_number: Option<String>,
    addenda02: Option<Box<MoovIoAchAddenda02>>,
    addenda98: Option<Box<MoovIoAchAddenda98>>,
    addenda98_refused: Option<Box<MoovIoAchAddenda98Refused>>,
    addenda99: Option<Box<MoovIoAchAddenda99>>,
    addenda99_contested: Option<Box<MoovIoAchAddenda99Contested>>,
    addenda99_dishonored: Option<Box<MoovIoAchAddenda99Dishonored>>,
}

impl MoovIoAchEntryDetail {
    fn set_trace_number(&mut self, odfi_identification: String, seq: i32) {
        let trace_number = self.string_field(&odfi_identification, 8) + &self.numeric_field(seq, 7);
        self.trace_number = Some(trace_number.clone());

        if let Some(ref mut addenda02) = self.addenda02 {
            addenda02.trace_number = Some(trace_number.clone());
        }
        if let Some(ref mut addenda98) = self.addenda98 {
            addenda98.trace_number = Some(trace_number.clone());
        }
        if let Some(ref mut addenda98_refused) = self.addenda98_refused {
            addenda98_refused.trace_number = Some(trace_number.clone());
        }
        if let Some(ref mut addenda99) = self.addenda99 {
            addenda99.trace_number = Some(trace_number.clone());
        }
        if let Some(ref mut addenda99_contested) = self.addenda99_contested {
            addenda99_contested.trace_number = Some(trace_number.clone());
        }
        if let Some(ref mut addenda99_dishonored) = self.addenda99_dishonored {
            addenda99_dishonored.trace_number = Some(trace_number);
        }
    }

    fn string_field(&self, s: &str, max: usize) -> String {
        let ln = s.chars().count();
        if ln > max {
            s.chars().take(max).collect()
        } else {
            let m = max - ln;
            if let Some(pad) = MOOV_IO_ACH_STRING_ZEROS.get(&m) {
                format!("{}{}", pad, s)
            } else {
                "0".repeat(m) + s
            }
        }
    }

    fn numeric_field(&self, n: i32, max: usize) -> String {
        let s = n.to_string();
        let l = s.len();
        if l > max {
            s.chars().skip(l - max).collect()
        } else {
            let m = max - l;
            if let Some(pad) = MOOV_IO_ACH_STRING_ZEROS.get(&m) {
                format!("{}{}", pad, s)
            } else {
                "0".repeat(m) + &s
            }
        }
    }
}

struct MoovIoAchAddenda02 {
    trace_number: Option<String>,
}

struct MoovIoAchAddenda98 {
    trace_number: Option<String>,
}

struct MoovIoAchAddenda98Refused {
    trace_number: Option<String>,
}

struct MoovIoAchAddenda99 {
    trace_number: Option<String>,
}

struct MoovIoAchAddenda99Contested {
    trace_number: Option<String>,
}

struct MoovIoAchAddenda99Dishonored {
    trace_number: Option<String>,
}

#[macro_use]
extern crate lazy_static;
