

use std::collections::HashMap;

static MOOV_IO_ACH_STRINGZEROS: once_cell::sync::Lazy<HashMap<usize, String>> = once_cell::sync::Lazy::new(|| {
    moov_io_ach_populate_map(94, "0".to_string())
});

struct MoovIoAchAdvEntryDetail {
    advice_routing_number: String,
    moov_io_ach_converters: MoovIoAchConverters,
}

struct MoovIoAchConverters;

impl MoovIoAchAdvEntryDetail {
    fn advice_routing_number_field(&self) -> String {
        self.moov_io_ach_converters.string_field(&self.advice_routing_number, 9)
    }
}

impl MoovIoAchConverters {
    fn string_field(&self, s: &str, max: u32) -> String {
        let ln = s.len() as u32;
        if ln > max {
            s[..max as usize].to_string()
        } else {
            let m = max - ln;
            let pad = MOOV_IO_ACH_STRINGZEROS.get(&(m as usize)).unwrap();
            format!("{}{}", pad, s)
        }
    }
}

fn moov_io_ach_populate_map(max: usize, zero: String) -> HashMap<usize, String> {
    let mut out = HashMap::with_capacity(max);
    for i in 0..max {
        out.insert(i, zero.repeat(i));
    }
    out
}

