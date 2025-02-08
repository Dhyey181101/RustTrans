
use std::collections::HashMap;

static mut MOOV_IO_ACH_STRING_ZEROS: Option<HashMap<i32, String>> = None;

fn moov_io_ach_populate_map(max: i32, zero: &str) -> HashMap<i32, String> {
    let mut out = HashMap::new();
    for i in 0..max {
        out.insert(i, zero.repeat(i as usize));
    }
    out
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
    fn set_trace_number(&mut self, odfi_identification: String, seq: i32) {
        let trace_number = self.string_field(&odfi_identification, 8) + &self.numeric_field(seq, 7);
        self.trace_number = trace_number.clone();

        if let Some(addenda) = &mut self.addenda02 {
            addenda.trace_number = trace_number.clone();
        }
        if let Some(addenda) = &mut self.addenda98 {
            addenda.trace_number = trace_number.clone();
        }
        if let Some(addenda) = &mut self.addenda98_refused {
            addenda.trace_number = trace_number.clone();
        }
        if let Some(addenda) = &mut self.addenda99 {
            addenda.trace_number = trace_number.clone();
        }
        if let Some(addenda) = &mut self.addenda99_contested {
            addenda.trace_number = trace_number.clone();
        }
        if let Some(addenda) = &mut self.addenda99_dishonored {
            addenda.trace_number = trace_number;
        }
    }

    fn string_field(&self, s: &str, max: u32) -> String {
        let ln = s.chars().count() as u32;
        if ln > max {
            s.chars().take(max as usize).collect()
        } else {
            let m = (max - ln) as i32;
            let pad = unsafe { MOOV_IO_ACH_STRING_ZEROS.as_ref().unwrap().get(&m).unwrap().clone() };
            pad + s
        }
    }

    fn numeric_field(&self, n: i32, max: u32) -> String {
        let s = n.to_string();
        let l = s.len() as u32;
        if l > max {
            s.chars().skip((l - max) as usize).collect()
        } else {
            let m = (max - l) as i32;
            let pad = unsafe { MOOV_IO_ACH_STRING_ZEROS.as_ref().unwrap().get(&m).unwrap().clone() };
            pad + &s
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
    unsafe {
        MOOV_IO_ACH_STRING_ZEROS = Some(moov_io_ach_populate_map(94, "0"));
    }
}
