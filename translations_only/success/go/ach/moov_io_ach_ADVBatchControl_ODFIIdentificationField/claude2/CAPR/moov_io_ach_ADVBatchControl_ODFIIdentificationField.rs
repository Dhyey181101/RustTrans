
use std::collections::HashMap;
use once_cell::sync::Lazy;

static MOOV_IO_ACH_STRINGZEROS: Lazy<HashMap<usize, String>> = Lazy::new(|| {
    moov_io_ach_populate_map(94, "0")
});

struct MoovIoAchAdvBatchControl {
    odfi_identification: String,
}

impl MoovIoAchAdvBatchControl {
    fn odfi_identification_field(&self) -> String {
        string_field(&self.odfi_identification, 8)
    }
}

fn string_field(s: &String, max: u32) -> String {
    let ln = s.len() as u32;
    if ln > max {
        s[..max as usize].to_string()
    } else {
        let m = (max - ln) as usize;
        match MOOV_IO_ACH_STRINGZEROS.get(&m) {
            Some(pad) => format!("{}{}", pad, s),
            None => "0".repeat(m) + s,
        }
    }
}

fn moov_io_ach_populate_map(max: usize, zero: &str) -> HashMap<usize, String> {
    let mut out = HashMap::with_capacity(max);
    for i in 0..max {
        out.insert(i, "0".repeat(i));
    }
    out
}

struct MoovIoAchConverters;

