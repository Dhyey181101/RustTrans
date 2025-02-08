
use std::collections::HashMap;
use once_cell::sync::Lazy;

static MOOV_IO_ACH_STRING_ZEROS: Lazy<HashMap<usize, String>> = Lazy::new(|| populate_map(94, "0"));

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

        if let Some(addenda02) = &mut self.addenda02 {
            addenda02.trace_number = self.trace_number.clone();
        }
        if let Some(addenda98) = &mut self.addenda98 {
            addenda98.trace_number = self.trace_number.clone();
        }
        if let Some(addenda98_refused) = &mut self.addenda98_refused {
            addenda98_refused.trace_number = self.trace_number.clone();
        }
        if let Some(addenda99) = &mut self.addenda99 {
            addenda99.trace_number = self.trace_number.clone();
        }
        if let Some(addenda99_contested) = &mut self.addenda99_contested {
            addenda99_contested.trace_number = self.trace_number.clone();
        }
        if let Some(addenda99_dishonored) = &mut self.addenda99_dishonored {
            addenda99_dishonored.trace_number = self.trace_number.clone();
        }
    }

    fn string_field(&self, s: &str, max: u32) -> String {
        let len = s.len() as u32;
        if len > max {
            s[..max as usize].to_string()
        } else {
            let missing = max - len;
            if let Some(pad) = MOOV_IO_ACH_STRING_ZEROS.get(&(missing as usize)) {
                format!("{}{}", pad, s)
            } else {
                format!("{}{}", "0".repeat(missing as usize), s)
            }
        }
    }

    fn numeric_field(&self, n: i32, max: u32) -> String {
        let s = n.to_string();
        let len = s.len() as u32;
        if len > max {
            s[s.len() - max as usize..].to_string()
        } else {
            let missing = max - len;
            if let Some(pad) = MOOV_IO_ACH_STRING_ZEROS.get(&(missing as usize)) {
                format!("{}{}", pad, s)
            } else {
                format!("{}{}", "0".repeat(missing as usize), s)
            }
        }
    }
}

fn populate_map(max: usize, zero: &str) -> HashMap<usize, String> {
    let mut map = HashMap::with_capacity(max);
    for i in 0..max {
        map.insert(i, zero.repeat(i));
    }
    map
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

