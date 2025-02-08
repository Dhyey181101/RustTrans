
use std::collections::HashMap;
use once_cell::sync::Lazy;

static MOOV_IO_ACH_STRINGZEROS: Lazy<HashMap<i32, String>> = Lazy::new(|| populate_map(94, "0".to_string()));

struct MoovIoAchIatEntryDetail {
    amount: i32,
}

impl MoovIoAchIatEntryDetail {
    fn amount_field(&self) -> String {
        numeric_field(self.amount, 10)
    }
}

struct MoovIoAchConverters;

fn numeric_field(n: i32, max: u32) -> String {
    let s = n.to_string();
    if s.len() as u32 > max {
        s[s.len()-max as usize..].to_string()
    } else {
        let m = max - s.len() as u32;
        match MOOV_IO_ACH_STRINGZEROS.get(&(m as i32)) {
            Some(pad) => format!("{}{}", pad, s),
            None => "0".repeat(m as usize) + &s,
        }
    }
}

fn populate_map(max: i32, zero: String) -> HashMap<i32, String> {
    let mut map = HashMap::new();
    for i in 0..max {
        map.insert(i, zero.repeat(i as usize));
    }
    map
}

