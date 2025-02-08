
use std::collections::HashMap;
use once_cell::sync::Lazy;

static MOOV_IO_ACH_STRING_ZEROS: Lazy<HashMap<usize, String>> = Lazy::new(|| moov_io_ach_populate_map(94, "0".to_string()));

struct MoovIoAchIatBatchHeader {
    odfi_identification: String,
}

fn odfi_identification_field(iat_bh: &Box<MoovIoAchIatBatchHeader>) -> String {
    string_field(&iat_bh.odfi_identification, 8)
}

fn string_field(s: &str, max: u32) -> String {
    let ln = s.chars().count() as u32;
    if ln > max {
        return s[..max as usize].to_string();
    }

    let m = (max - ln) as usize;
    MOOV_IO_ACH_STRING_ZEROS.get(&m).unwrap_or(&"".to_string()).to_string() + s
}

fn moov_io_ach_populate_map(max: usize, zero: String) -> HashMap<usize, String> {
    let mut out = HashMap::with_capacity(max);
    for i in 0..max {
        out.insert(i, "0".repeat(i) + &zero);
    }
    out
}
