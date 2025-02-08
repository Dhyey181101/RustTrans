
use std::collections::HashMap;

static mut MOOV_IO_ACH_STRING_ZEROS: Option<Box<HashMap<i32, String>>> = None;

fn moov_io_ach_populate_map(max: i32, zero: &str) -> HashMap<i32, String> {
    let mut out = HashMap::with_capacity(max as usize);
    for i in 0..max {
        out.insert(i, zero.repeat(i as usize));
    }
    out
}

struct MoovIoAchAddenda98 {
    original_trace: String,
}

impl MoovIoAchAddenda98 {
    fn original_trace_field(&self) -> String {
        let ln = self.original_trace.chars().count() as u32;
        if ln > 15 {
            self.original_trace.chars().take(15 as usize).collect()
        } else {
            let m = (15 - ln) as i32;
            let pad = unsafe {
                MOOV_IO_ACH_STRING_ZEROS
                    .as_ref()
                    .unwrap()
                    .get(&m)
                    .unwrap_or(&String::new())
                    .clone()
            };
            format!("{}{}", pad, self.original_trace)
        }
    }
}

fn main() {
    unsafe {
        MOOV_IO_ACH_STRING_ZEROS = Some(Box::new(moov_io_ach_populate_map(94, "0")));
    }
}
