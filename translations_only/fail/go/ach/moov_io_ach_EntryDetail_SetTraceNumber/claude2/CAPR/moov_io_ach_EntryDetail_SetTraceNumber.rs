
use std::collections::HashMap;
use std::str;
use once_cell::sync::Lazy;

static MOOV_IO_ACH_STRING_ZEROS: Lazy<HashMap<u32, String>> = Lazy::new(|| populate_map(94, "0".to_string()));

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
        let trace_number = MoovIoAchConverters::string_field(odfi_identification, 8) + &MoovIoAchConverters::numeric_field(seq, 7);
        self.trace_number = trace_number.clone();

        if let Some(ref mut addenda02) = self.addenda02 {
            addenda02.trace_number = self.trace_number.clone();
        }
        if let Some(ref mut addenda98) = self.addenda98 {
            addenda98.trace_number = self.trace_number.clone();
        }
        if let Some(ref mut addenda98_refused) = self.addenda98_refused {
            addenda98_refused.trace_number = self.trace_number.clone();
        }
        if let Some(ref mut addenda99) = self.addenda99 {
            addenda99.trace_number = self.trace_number.clone();
        }
        if let Some(ref mut addenda99_contested) = self.addenda99_contested {
            addenda99_contested.trace_number = self.trace_number.clone();
        }
        if let Some(ref mut addenda99_dishonored) = self.addenda99_dishonored {
            addenda99_dishonored.trace_number = self.trace_number.clone();
        }
    }
}

struct MoovIoAchConverters;

impl MoovIoAchConverters {
    fn string_field(s: &str, max: u32) -> String {
        let len = s.len() as u32;
        if len > max {
            s[..max as usize].to_string()
        } else {
            let pad = MOOV_IO_ACH_STRING_ZEROS.get(&(max - len)).unwrap();
            format!("{}{}", pad, s)
        }
    }

    fn numeric_field(n: i32, max: u32) -> String {
        let s = n.to_string();
        let len = s.len() as u32;
        if len > max {
            s[(len - max) as usize..].to_string()
        } else {
            let pad = MOOV_IO_ACH_STRING_ZEROS.get(&(max - len)).unwrap();
            format!("{}{}", pad, s)
        }
    }
}

fn populate_map(max: u32, zero: String) -> HashMap<u32, String> {
    let mut map = HashMap::with_capacity(max as usize);
    for i in 0..max {
        map.insert(i, zero.repeat(i as usize));
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

