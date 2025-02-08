
use std::collections::HashMap;

static mut MOOV_IO_ACH_STRING_ZEROS: Option<HashMap<usize, String>> = None;

fn moov_io_ach_populate_map(max: usize, zero: &str) -> HashMap<usize, String> {
    let mut out = HashMap::with_capacity(max);
    for i in 0..max {
        out.insert(i, zero.repeat(i));
    }
    out
}

struct MoovIoAchAdvFileControl {
    entry_addenda_count: usize,
}

impl MoovIoAchAdvFileControl {
    fn entry_addenda_count_field(&self) -> String {
        self.numeric_field(self.entry_addenda_count, 8)
    }
}

impl MoovIoAchAdvFileControl {
    fn numeric_field(&self, n: usize, max: u32) -> String {
        let mut s = n.to_string();
        if s.len() > max as usize {
            s.drain(..s.len() - max as usize);
            s
        } else {
            let m = max as usize - s.len();
            let pad = unsafe {
                if MOOV_IO_ACH_STRING_ZEROS.is_none() {
                    MOOV_IO_ACH_STRING_ZEROS = Some(moov_io_ach_populate_map(94, "0"));
                }
                MOOV_IO_ACH_STRING_ZEROS.as_ref().unwrap().get(&m).unwrap_or(&"".to_string()).clone()
            };
            pad + &s
        }
    }
}

struct MoovIoAchConverters;
