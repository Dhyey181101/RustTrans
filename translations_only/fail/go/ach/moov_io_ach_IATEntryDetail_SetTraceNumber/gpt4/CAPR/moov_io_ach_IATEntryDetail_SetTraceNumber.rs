
use std::collections::HashMap;

static mut MOOV_IO_ACH_STRING_ZEROS: Option<HashMap<i32, String>> = None;

fn moov_io_ach_populate_map(max: i32, zero: &str) -> HashMap<i32, String> {
    let mut out = HashMap::new();
    for i in 0..max {
        out.insert(i, zero.repeat(i as usize));
    }
    out
}

struct MoovIoAchIATEntryDetail {
    trace_number: String,
}

impl MoovIoAchIATEntryDetail {
    fn set_trace_number(&mut self, odfi_identification: String, seq: i32) {
        self.trace_number = self.string_field(&odfi_identification, 8) + &self.numeric_field(seq, 7);
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

fn main() {
    unsafe {
        MOOV_IO_ACH_STRING_ZEROS = Some(moov_io_ach_populate_map(94, "0"));
    }
}
