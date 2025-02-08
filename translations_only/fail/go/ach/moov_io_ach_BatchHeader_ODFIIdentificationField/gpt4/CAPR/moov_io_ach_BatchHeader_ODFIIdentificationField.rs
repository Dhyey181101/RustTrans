
use std::collections::HashMap;

static mut MOOV_IO_ACH_STRING_ZEROS: Option<Box<HashMap<i32, String>>> = None;

struct MoovIoAchBatchHeader {
    odfi_identification: String,
}

struct MoovIoAchConverters;

impl MoovIoAchBatchHeader {
    fn odfi_identification_field(&self) -> String {
        MoovIoAchConverters::string_field(&self.odfi_identification, 8)
    }
}

impl MoovIoAchConverters {
    fn string_field(s: &str, max: u32) -> String {
        let ln = s.chars().count() as u32;
        if ln > max {
            return s.chars().take(max as usize).collect();
        }

        let m = (max - ln) as i32;
        let pad = unsafe {
            MOOV_IO_ACH_STRING_ZEROS
                .as_ref()
                .unwrap()
                .get(&m)
                .unwrap_or(&String::new())
                .clone()
        };
        format!("{}{}", pad, s)
    }
}

fn moov_io_ach_populate_map(max: i32, zero: &str) -> HashMap<i32, String> {
    let mut out = HashMap::with_capacity(max as usize);
    for i in 0..max {
        out.insert(i, zero.repeat(i as usize));
    }
    out
}

fn main() {
    unsafe {
        MOOV_IO_ACH_STRING_ZEROS = Some(Box::new(moov_io_ach_populate_map(94, "0")));
    }
}
