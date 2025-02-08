
use std::collections::HashMap;

static mut MOOV_IO_ACH_STRING_ZEROS: Option<Box<HashMap<i32, String>>> = None;

fn moov_io_ach_populate_map(max: i32, zero: &str) -> HashMap<i32, String> {
    let mut out = HashMap::new();
    for i in 0..max {
        out.insert(i, zero.repeat(i as usize));
    }
    out
}

struct MoovIoAchEntryDetail {
    trace_number: String,
}

impl MoovIoAchEntryDetail {
    fn trace_number_field(&self) -> String {
        let max = 15;
        let ln = self.trace_number.chars().count() as u32;
        if ln > max {
            self.trace_number.chars().take(max as usize).collect()
        } else {
            let m = (max - ln) as i32;
            let pad = unsafe {
                MOOV_IO_ACH_STRING_ZEROS
                    .as_ref()
                    .unwrap()
                    .get(&m)
                    .unwrap()
                    .clone()
            };
            format!("{}{}", pad, self.trace_number)
        }
    }
}

fn main() {
    unsafe {
        MOOV_IO_ACH_STRING_ZEROS = Some(Box::new(moov_io_ach_populate_map(94, "0")));
    }
}
