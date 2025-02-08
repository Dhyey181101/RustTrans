
use std::collections::HashMap;
use std::string::String;
use once_cell::sync::Lazy;

static MOOV_IO_ACH_STRING_ZEROS: Lazy<HashMap<i32, String>> = Lazy::new(|| populate_map(94, "0".to_owned()));

struct MoovIoAchAddenda99 {
    trace_number: String,
}

impl MoovIoAchAddenda99 {
    fn trace_number_field(&self) -> String {
        string_field(&self.trace_number, 15)
    }
}

fn string_field(s: &String, max: u32) -> String {
    let ln = s.len() as u32;
    if ln > max {
        s[..max as usize].to_owned()
    } else {
        let m = (max - ln) as i32;
        match MOOV_IO_ACH_STRING_ZEROS.get(&m) {
            Some(pad) => pad.to_owned() + s,
            None => "0".repeat(m as usize) + s,
        }
    }
}

fn populate_map(max: i32, zero: String) -> HashMap<i32, String> {
    let mut out = HashMap::with_capacity(max as usize);
    for i in 0..max {
        out.insert(i, "0".repeat(i as usize));
    }
    out
}

