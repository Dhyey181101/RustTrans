
use std::collections::HashMap;
use std::str;
use once_cell::sync::Lazy;

static MOOV_IO_ACH_STRING_ZEROS: Lazy<HashMap<usize, String>> = Lazy::new(|| {
    moov_io_ach_populate_map(94, "0".to_string())
});

fn moov_io_ach_batch_header_odfi_identification_field(bh: &Box<MoovIoAchBatchHeader>) -> String {
    bh.string_field(bh.odfi_identification.clone(), 8)
}

fn moov_io_ach_converters_string_field(s: String, max: u32) -> String {
    let ln = s.chars().count() as u32;
    if ln > max {
        return s.chars().take(max as usize).collect();
    }

    let m = (max - ln) as usize;
    let pad = MOOV_IO_ACH_STRING_ZEROS.get(&m).unwrap_or(&"".to_string()).clone();
    pad + &s
}

fn moov_io_ach_populate_map(max: usize, zero: String) -> HashMap<usize, String> {
    let mut out = HashMap::with_capacity(max);
    for i in 0..max {
        out.insert(i, zero.repeat(i));
    }
    out
}

struct MoovIoAchBatchHeader {
    odfi_identification: String,
}

impl MoovIoAchBatchHeader {
    fn string_field(&self, s: String, max: u32) -> String {
        moov_io_ach_converters_string_field(s, max)
    }
}
