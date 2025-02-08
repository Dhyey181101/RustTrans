
use std::collections::HashMap;
use std::str::FromStr;
use once_cell::sync::Lazy;

static MOV_IO_ACH_STRING_ZEROS: Lazy<HashMap<i32, String>> = Lazy::new(|| populate_map(94, "0"));

struct MoovIoAchIatEntryDetail {
    addenda_records: i32,
}

impl MoovIoAchIatEntryDetail {
    fn addenda_records_field(&self) -> String {
        numeric_field(self.addenda_records, 4)
    }
}

struct MoovIoAchConverters;

fn numeric_field(n: i32, max: u32) -> String {
    let s = n.to_string();
    if s.len() as u32 > max {
        s[s.len() - max as usize..].to_string()
    } else {
        let m = max - s.len() as u32;
        let pad = MOV_IO_ACH_STRING_ZEROS.get(&(m as i32)).unwrap();
        format!("{}{}", pad, s)
    }
}

fn populate_map(max: i32, zero: &str) -> HashMap<i32, String> {
    let mut map = HashMap::new();
    for i in 0..max {
        map.insert(i, zero.repeat(i as usize));
    }
    map
}

