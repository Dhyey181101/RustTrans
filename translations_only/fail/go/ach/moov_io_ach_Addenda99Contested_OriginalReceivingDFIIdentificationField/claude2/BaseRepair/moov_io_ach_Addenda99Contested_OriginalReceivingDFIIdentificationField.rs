
use std::collections::HashMap;
use once_cell::sync::Lazy;

static MOOV_IO_ACH_STRINGZEROS: Lazy<HashMap<usize, String>> = Lazy::new(|| {
    let mut out = HashMap::with_capacity(94);
    for i in 0..94 {
        out.insert(i, "0".repeat(i));
    }
    out
});

struct MoovIoAchAddenda99Contested {
    original_receiving_dfi_identification: String,
    moov_io_ach_converters: MoovIoAchConverters,
}

struct MoovIoAchConverters;

impl MoovIoAchAddenda99Contested {
    fn original_receiving_dfi_identification_field(&self) -> String {
        string_field(&self.original_receiving_dfi_identification, 8)
    }
}

fn string_field(s: &str, max: u32) -> String {
    let mut result = String::with_capacity(max as usize);
    result.push_str(s);
    while result.len() < max as usize {
        result.push('0');
    }
    result
}

