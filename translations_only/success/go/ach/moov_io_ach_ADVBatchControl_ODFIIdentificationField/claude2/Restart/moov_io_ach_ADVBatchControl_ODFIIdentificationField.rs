
use std::collections::HashMap;
use once_cell::sync::Lazy;

static MOOV_IO_ACH_STRINGZEROS: Lazy<HashMap<usize, String>> = Lazy::new(|| {
    moov_io_ach_populate_map(94, "0".to_string())
});

struct MoovIoAchAdvBatchControl {
    odfi_identification: String,
}

impl MoovIoAchAdvBatchControl {
    fn odfi_identification_field(&self) -> String {
        string_field(&self.odfi_identification, 8)
    }
}

struct MoovIoAchConverters;

fn string_field(s: &String, max: u32) -> String {
    let ln = s.len() as u32;
    if ln > max {
        s[..max as usize].to_string()
    } else {
        let m = max - ln;
        match MOOV_IO_ACH_STRINGZEROS.get(&(m as usize)) {
            Some(pad) => format!("{}{}", pad, s),
            None => "0".repeat(m as usize) + s,
        }
    }
}

fn moov_io_ach_populate_map(max: usize, zero: String) -> HashMap<usize, String> {
    let mut out = HashMap::with_capacity(max);
    for i in 0..max {
        out.insert(i, "0".repeat(i));
    }
    out
}

