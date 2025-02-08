
use std::collections::HashMap;
use std::str;
use once_cell::sync::Lazy;

static MOOV_IO_ACH_STRING_ZEROS: Lazy<HashMap<usize, Box<str>>> = Lazy::new(|| moov_io_ach_populate_map(94, Box::from("0")));

struct MoovIoAchBatchHeader {
    odfi_identification: String,
}

impl MoovIoAchBatchHeader {
    fn odfi_identification_field(&self) -> String {
        let converters = MoovIoAchConverters {};
        converters.string_field(&self.odfi_identification, 8)
    }
}

struct MoovIoAchConverters {}

impl MoovIoAchConverters {
    fn string_field(&self, s: &str, max: u32) -> String {
        let ln = s.chars().count() as u32;
        if ln > max {
            return s[..max as usize].to_string();
        }

        let m = (max - ln) as usize;
        let empty_string = Box::from("".to_string());
        let pad = MOOV_IO_ACH_STRING_ZEROS.get(&m).unwrap_or(&empty_string);
        pad.to_string() + s
    }
}

fn moov_io_ach_populate_map(max: usize, zero: Box<str>) -> HashMap<usize, Box<str>> {
    let mut out = HashMap::with_capacity(max);
    for i in 0..max {
        out.insert(i, Box::from(zero.repeat(i)));
    }
    out
}
