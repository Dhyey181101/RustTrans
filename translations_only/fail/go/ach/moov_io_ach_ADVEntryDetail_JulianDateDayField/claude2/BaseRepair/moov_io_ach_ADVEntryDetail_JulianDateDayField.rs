
use std::collections::HashMap;
use std::str::FromStr;
use once_cell::sync::Lazy;

static MOOV_IO_ACH_STRINGZEROS: Lazy<HashMap<u32, String>> = Lazy::new(|| populate_map(94, "0".to_string()));

struct MoovIoAchConverters;

struct MoovIoAchAdvEntryDetail {
    julian_day: i32,
}

impl MoovIoAchAdvEntryDetail {
    fn julian_date_day_field(&self) -> String {
        numeric_field(self.julian_day as u32, 3)
    }
}

fn numeric_field(n: u32, max: u32) -> String {
    let s = n.to_string();
    if s.len() as u32 > max {
        s[s.len() - max as usize..].to_string()
    } else {
        let m = max - s.len() as u32;
        let pad = MOOV_IO_ACH_STRINGZEROS.get(&m).unwrap();
        format!("{}{}", pad, s)
    }
}

fn populate_map(max: i32, zero: String) -> HashMap<u32, String> {
    let mut out = HashMap::new();
    for i in 0..max {
        out.insert(i as u32, zero.repeat(i as usize));
    }
    out
}

