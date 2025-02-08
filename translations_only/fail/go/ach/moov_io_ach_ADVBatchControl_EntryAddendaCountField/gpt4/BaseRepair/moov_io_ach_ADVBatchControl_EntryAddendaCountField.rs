
use std::collections::HashMap;

static mut MOOV_IO_ACH_STRING_ZEROS: Option<HashMap<i32, String>> = None;

fn moov_io_ach_populate_map(max: i32, zero: &str) -> HashMap<i32, String> {
    let mut out = HashMap::with_capacity(max as usize);
    for i in 0..max {
        out.insert(i, zero.repeat(i as usize));
    }
    out
}

struct MoovIoAchADVBatchControl {
    entry_addenda_count: i32,
}

impl MoovIoAchADVBatchControl {
    fn entry_addenda_count_field(&self) -> String {
        self.numeric_field(self.entry_addenda_count, 6)
    }

    fn numeric_field(&self, n: i32, max: u32) -> String {
        let s = n.to_string();
        let l = s.len() as u32;
        if l > max {
            s.chars().skip((l - max) as usize).collect()
        } else {
            let m = (max - l) as i32;
            let pad = unsafe {
                MOOV_IO_ACH_STRING_ZEROS
                    .as_ref()
                    .unwrap()
                    .get(&m)
                    .unwrap()
                    .clone()
            };
            format!("{}{}", pad, s)
        }
    }
}

struct MoovIoAchConverters {}

fn main() {
    unsafe {
        MOOV_IO_ACH_STRING_ZEROS = Some(moov_io_ach_populate_map(94, "0"));
    }
}
