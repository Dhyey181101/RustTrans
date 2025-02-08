
use std::collections::HashMap;

static mut MOOV_IO_ACH_STRING_ZEROS: Option<Box<HashMap<i32, String>>> = None;

fn moov_io_ach_populate_map(max: i32, zero: &str) -> Box<HashMap<i32, String>> {
    let mut out = HashMap::new();
    for i in 0..max {
        out.insert(i, zero.repeat(i as usize));
    }
    Box::new(out)
}

struct MoovIoAchIATBatchHeader {
    odfi_identification: String,
}

struct MoovIoAchConverters;

impl MoovIoAchConverters {
    fn string_field(&self, s: &str, max: u32) -> String {
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

impl MoovIoAchIATBatchHeader {
    fn odfi_identification_field(&self) -> String {
        let converters = MoovIoAchConverters {};
        converters.string_field(&self.odfi_identification, 8)
    }
}

fn main() {
    unsafe {
        MOOV_IO_ACH_STRING_ZEROS = Some(moov_io_ach_populate_map(94, "0"));
    }
}
