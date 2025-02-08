
use std::collections::HashMap;
use once_cell::sync::Lazy;

static MOOV_IO_ACH_STRING_ZEROS: Lazy<HashMap<usize, String>> = Lazy::new(|| {
    populate_map(94, "0".to_string())
});

struct MoovIoAchAddenda99Dishonored {
    original_receiving_dfi_identification: String,
    moov_io_ach_converters: MoovIoAchConverters,
}

struct MoovIoAchConverters;

impl MoovIoAchAddenda99Dishonored {
    fn original_receiving_dfi_identification_field(&self) -> String {
        string_field(&self.original_receiving_dfi_identification, 8)
    }
}

fn string_field(s: &String, max: u32) -> String {
    let ln = s.chars().count() as u32;
    if ln > max {
        s[..max as usize].to_string()
    } else {
        let mut pad = String::new();
        for _ in 0..(max - ln) {
            pad.push('0');
        }
        pad + s
    }
}

fn populate_map(max: usize, zero: String) -> HashMap<usize, String> {
    let mut map = HashMap::with_capacity(max);
    for i in 0..max {
        map.insert(i, zero.repeat(i));
    }
    map
}

