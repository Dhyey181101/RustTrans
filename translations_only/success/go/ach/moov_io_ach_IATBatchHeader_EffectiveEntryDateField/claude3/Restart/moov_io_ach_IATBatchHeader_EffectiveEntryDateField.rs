
use std::collections::HashMap;

static MOOV_IO_ACH_STRING_ZEROS: Lazy<HashMap<usize, Box<str>>> = Lazy::new(|| {
    let mut map = HashMap::with_capacity(94);
    for i in 0..94 {
        map.insert(i, Box::from("0".repeat(i)));
    }
    map
});

struct MoovIoAchIatbatchheader {
    effective_entry_date: String,
}

impl MoovIoAchIatbatchheader {
    fn effective_entry_date_field(&self) -> String {
        string_field(&self.effective_entry_date, 6)
    }
}

fn string_field(s: &str, max: u32) -> String {
    let ln = s.len() as u32;
    if ln > max {
        return s.chars().take(max as usize).collect();
    }

    let m = (max - ln) as usize;
    MOOV_IO_ACH_STRING_ZEROS.get(&m).unwrap().to_string() + s
}

use once_cell::sync::Lazy;
