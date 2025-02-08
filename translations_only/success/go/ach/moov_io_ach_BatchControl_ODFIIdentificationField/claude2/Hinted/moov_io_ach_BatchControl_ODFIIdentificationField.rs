
use std::collections::HashMap;
use once_cell::sync::Lazy;

static MOOV_IO_ACH_STRING_ZEROS: Lazy<HashMap<usize, String>> = Lazy::new(|| {
    moov_io_ach_populate_map(94, "0".to_string())
});

struct MoovIoAchBatchControl {
    odfi_identification: String,
}

impl MoovIoAchBatchControl {
    fn odfi_identification_field(&self) -> String {
        moov_io_ach_string_field(self.odfi_identification.as_str(), 8)
    }
}

fn moov_io_ach_string_field(s: &str, max: u32) -> String {
    let ln = s.len() as u32;
    if ln > max {
        s[..max as usize].to_string()
    } else {
        let m = (max - ln) as usize;
        MOOV_IO_ACH_STRING_ZEROS.get(&m).cloned().unwrap_or_else(|| "0".repeat(m)) + s
    }
}

fn moov_io_ach_populate_map(max: i32, zero: String) -> HashMap<usize, String> {
    let mut out = HashMap::with_capacity(max as usize);
    for i in 0..max {
        out.insert(i as usize, "0".repeat(i as usize));
    }
    out
}

struct MoovIoAchConverters;

