
use std::collections::HashMap;

static mut MOOV_IO_ACH_STRING_ZEROS: Option<HashMap<i32, String>> = None;

fn moov_io_ach_populate_map(max: i32, zero: &str) -> HashMap<i32, String> {
    let mut out = HashMap::with_capacity(max as usize);
    for i in 0..max {
        out.insert(i, zero.repeat(i as usize));
    }
    out
}

struct MoovIoAchIATBatchHeader {
    odfi_identification: String,
    converters: Box<MoovIoAchConverters>,
}

impl MoovIoAchIATBatchHeader {
    fn new(odfi_identification: String) -> Self {
        Self {
            odfi_identification,
            converters: Box::new(MoovIoAchConverters {}),
        }
    }

    fn odfi_identification_field(&self) -> String {
        self.converters.string_field(&self.odfi_identification, 8)
    }
}

struct MoovIoAchConverters;

impl MoovIoAchConverters {
    fn string_field(&self, s: &str, max: u32) -> String {
        let ln = s.chars().count() as u32;
        if ln > max {
            s.chars().take(max as usize).collect()
        } else {
            let m = (max - ln) as i32;
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

fn main() {
    unsafe {
        MOOV_IO_ACH_STRING_ZEROS = Some(moov_io_ach_populate_map(94, "0"));
    }
}
