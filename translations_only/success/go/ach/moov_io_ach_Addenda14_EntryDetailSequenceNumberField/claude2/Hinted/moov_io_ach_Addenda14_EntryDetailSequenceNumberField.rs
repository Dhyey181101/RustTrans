
use std::collections::HashMap;
use once_cell::sync::Lazy;

static MOOV_IO_ACH_STRINGZEROS: Lazy<HashMap<i32, String>> = Lazy::new(|| {
    let mut map = HashMap::new();
    map.insert(0, "0".to_string());
    map
});

struct Addenda14 {
    entry_detail_sequence_number: i32,
}

impl Addenda14 {
    fn entry_detail_sequence_number_field(&self) -> String {
        numeric_field(self.entry_detail_sequence_number, 7)
    }
}

fn numeric_field(n: i32, max: u32) -> String {
    let s = n.to_string();
    if s.len() as u32 > max {
        s[s.len() - max as usize..].to_string()
    } else {
        let m = max - s.len() as u32;
        let pad = MOOV_IO_ACH_STRINGZEROS.get(&0).unwrap().clone();
        pad.repeat(m as usize) + &s  
    }
}

struct Converters;
