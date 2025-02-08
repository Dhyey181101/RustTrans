
use std::collections::HashMap;
use std::str::FromStr;
use once_cell::sync::Lazy;

static MOOV_IO_ACH_STRINGZEROS: Lazy<HashMap<usize, String>> = Lazy::new(|| {
    let mut map = HashMap::new();
    for i in 0..94 {
        map.insert(i, "0".repeat(i));
    }
    map
});

struct Addenda13 {
    entry_detail_sequence_number: i32,
}

impl Addenda13 {
    fn entry_detail_sequence_number_field(&self) -> String {
        numeric_field(self.entry_detail_sequence_number, 7)
    }
}

fn numeric_field(n: i32, max: u32) -> String {
    let s = n.to_string();
    if s.len() as u32 > max {
        s[s.len()-max as usize..].to_string()
    } else {
        let m = (max - s.len() as u32) as usize;
        MOOV_IO_ACH_STRINGZEROS.get(&m).cloned().unwrap_or_else(|| "0".repeat(m)) + &s
    }
}

struct Converters;

