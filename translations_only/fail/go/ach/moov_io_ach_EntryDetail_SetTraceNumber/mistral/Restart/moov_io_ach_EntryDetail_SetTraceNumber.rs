

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
                format!("{:<width$}", s, width = max)
            }
        }
    }

    fn numeric_field(&self, n: i32, max: usize) -> String {
        let s = n.to_string();
        let l = s.len();
        if l > max {
            s[l - max..].to_string()
        } else {
            let m = max - l;
            let pad = &ZEROS[..m];
            if let Some(pad) = str::from_utf8(pad).ok() {
                format!("{}{}", pad, s)
            } else {
                format!("{:<width$}", s, width = max)
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
        let trace_number = moov_io_ach_converters.string_field(odfi_identification, 8) + &moov_io_ach_converters.numeric_field(seq, 7);
        self.trace_number = trace_number;

        if let Some(ref mut addenda02) = self.addenda02 {
            addenda02.set_trace_number(odfi_identification, seq);
        }
        if let Some(ref mut addenda98) = self.addenda98 {
            addenda98.set_trace_number(odfi_identification, seq);
        }
        if let Some(ref mut addenda98_refused) = self.addenda98_refused {
            addenda98_refused.set_trace_number(odfi_identification, seq);
        }
        if let Some(ref mut addenda99) = self.addenda99 {
            addenda99.set_trace_number(odfi_identification, seq);
        }
        if let Some(ref mut addenda99_contested) = self.addenda99_contested {
            addenda99_contested.set_trace_number(odfi_identification, seq);
        }
        if let Some(ref mut addenda99_dishonored) = self.addenda99_dishonored {
            addenda99_dishonored.set_trace_number(odfi_identification, seq);
        }
    }
}

struct MoovIoAchAddenda02 {
    trace_number: String,
}

impl MoovIoAchAddenda02 {
    fn new() -> MoovIoAchAddenda02 {
        MoovIoAchAddenda02 {
            trace_number: String::new(),
        }
    }

    fn set_trace_number(&mut self, odfi_identification: &str, seq: i32) {
        let moov_io_ach_converters = MoovIoAchConverters;
        let trace_number = moov_io_ach_converters.string_field(odfi_identification, 8) + &moov_io_ach_converters.numeric_field(seq, 7);
        self.trace_number = trace_number;
    }
}

struct MoovIoAchAddenda98 {
    trace_number: String,
}

impl MoovIoAchAddenda98 {
    fn new() -> MoovIoAchAddenda98 {
        MoovIoAchAddenda98 {
            trace_number: String::new(),
        }
    }

    fn set_trace_number(&mut self, odfi_identification: &str, seq: i32) {
        let moov_io_ach_converters = MoovIoAchConverters;
        let trace_number = moov_io_ach_converters.string_field(odfi_identification, 8) + &moov_io_ach_converters.numeric_field(seq, 7);
        self.trace_number = trace_number;
    }
}

struct MoovIoAchAddenda98Refused {
    trace_number: String,
}

impl MoovIoAchAddenda98Refused {
    fn new() -> MoovIoAchAddenda98Refused {
        MoovIoAchAddenda98Refused {
            trace_number: String::new(),
        }
    }

    fn set_trace_number(&mut self, odfi_identification: &str, seq: i32) {
        let moov_io_ach_converters = MoovIoAchConverters;
        let trace_number = moov_io_ach_converters.string_field(odfi_identification, 8) + &moov_io_ach_converters.numeric_field(seq, 7);
        self.trace_number = trace_number;
    }
}

struct MoovIoAchAddenda99 {
    trace_number: String,
}

impl MoovIoAchAddenda99 {
    fn new() -> MoovIoAchAddenda99 {
        MoovIoAchAddenda99 {
            trace_number: String::new(),
        }
    }

    fn set_trace_number(&mut self, odfi_identification: &str, seq: i32) {
        let moov_io_ach_converters = MoovIoAchConverters;
        let trace_number = moov_io_ach_converters.string_field(odfi_identification, 8) + &moov_io_ach_converters.numeric_field(seq, 7);
        self.trace_number = trace_number;
    }
}

struct MoovIoAchAddenda99Contested {
    trace_number: String,
}

impl MoovIoAchAddenda99Contested {
    fn new() -> MoovIoAchAddenda99Contested {
        MoovIoAchAddenda99Contested {
            trace_number: String::new(),
        }
    }

    fn set_trace_number(&mut self, odfi_identification: &str, seq: i32) {
        let moov_io_ach_converters = MoovIoAchConverters;
        let trace_number = moov_io_ach_converters.string_field(odfi_identification, 8) + &moov_io_ach_converters.numeric_field(seq, 7);
        self.trace_number = trace_number;
    }
}

struct MoovIoAchAddenda99Dishonored {
    trace_number: String,
}

impl MoovIoAchAddenda99Dishonored {
    fn new() -> MoovIoAchAddenda99Dishonored {
        MoovIoAchAddenda99Dishonored {
            trace_number: String::new(),
        }
    }

    fn set_trace_number(&mut self, odfi_identification: &str, seq: i32) {
        let moov_io_ach_converters = MoovIoAchConverters;
        let trace_number = moov_io_ach_converters.string_field(odfi_identification, 8) + &moov_io_ach_converters.numeric_field(seq, 7);
        self.trace_number = trace_number;
    }
}

impl fmt::Display for MoovIoAchEntryDetail {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.trace_number)
    }
}

impl fmt::Display for MoovIoAchAddenda02 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.trace_number)
    }
}

impl fmt::Display for MoovIoAchAddenda98 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.trace_number)
    }
}

impl fmt::Display for MoovIoAchAddenda98Refused {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.trace_number)
    }
}

impl fmt::Display for MoovIoAchAddenda99 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.trace_number)
    }
}

impl fmt::Display for MoovIoAchAddenda99Contested {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.trace_number)
    }
}

impl fmt::Display for MoovIoAchAddenda99Dishonored {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.trace_number)
    }
}

