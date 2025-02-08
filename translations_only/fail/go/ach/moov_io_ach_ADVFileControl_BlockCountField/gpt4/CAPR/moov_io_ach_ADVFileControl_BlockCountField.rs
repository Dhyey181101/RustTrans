
use std::collections::HashMap;

static mut MOOV_IO_ACH_STRING_ZEROS: Option<HashMap<i32, String>> = None;

fn moov_io_ach_populate_map(max: i32, zero: &str) -> HashMap<i32, String> {
    let mut out = HashMap::new();
    for i in 0..max {
        out.insert(i, zero.repeat(i as usize));
    }
    out
}

struct MoovIoAchADVFileControl {
    block_count: i32,
}

impl MoovIoAchADVFileControl {
    fn block_count_field(&self) -> String {
        self.numeric_field(self.block_count, 6)
    }

    fn numeric_field(&self, n: i32, max: u32) -> String {
        let s = n.to_string();
        if s.len() as u32 > max {
            s.chars().skip((s.len() as u32 - max) as usize).collect()
        } else {
            let m = (max - s.len() as u32) as i32;
            let pad = unsafe {
                MOOV_IO_ACH_STRING_ZEROS
                    .as_ref()
                    .unwrap()
                    .get(&m)
                    .unwrap_or(&"".to_string())
                    .clone()
            };
            format!("{}{}", pad, s)
        }
    }
}

struct MoovIoAchConverters;

fn main() {
    unsafe {
        MOOV_IO_ACH_STRING_ZEROS = Some(moov_io_ach_populate_map(94, "0"));
    }
}
