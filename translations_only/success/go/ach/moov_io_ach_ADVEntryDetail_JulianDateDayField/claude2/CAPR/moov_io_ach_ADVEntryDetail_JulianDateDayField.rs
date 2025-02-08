

use std::collections::HashMap;
use std::str::FromStr;
use once_cell::sync::Lazy;

static MOV_IO_ACH_STRINGZEROS: Lazy<HashMap<u32, String>> = Lazy::new(|| populate_map(94, "0".to_string()));

struct MovIoAchConverters;

struct MovIoAchAdvEntryDetail {
    julian_day: i32,
}

impl MovIoAchAdvEntryDetail {
    fn julian_date_day_field(&self) -> String {
        numeric_field(self.julian_day, 3)
    }
}

fn numeric_field(n: i32, max: u32) -> String {
    let s = n.to_string();
    if s.len() as u32 > max {
        s[s.len() - max as usize..].to_string()
    } else {
        let m = max - s.len() as u32;
        let pad = MOV_IO_ACH_STRINGZEROS.get(&m).unwrap().to_string();
        pad + &s
    }
}

fn populate_map(max: u32, zero: String) -> HashMap<u32, String> {
    let mut out = HashMap::with_capacity(max as usize);
    for i in 0..max {
        out.insert(i, zero.repeat(i as usize));
    }
    out
}


