
use std::collections::HashMap;
use std::str::FromStr;
use once_cell::sync::Lazy;

static MOOV_IO_ACH_STRINGZEROS: Lazy<HashMap<i32, String>> = Lazy::new(|| populate_map(94, "0".to_string()));

struct MoovIoAchConverters;

struct MoovIoAchAdvEntryDetail {
    sequence_number: i32,
}

impl MoovIoAchAdvEntryDetail {
    fn sequence_number_field(&self) -> String {
        numeric_field(self.sequence_number, 4)
    }
}

fn numeric_field(n: i32, max: u32) -> String {
    let s = n.to_string();
    let l = s.len() as u32;

    if l > max {
        s[s.len() - max as usize..].to_string()
    } else {
        let m = max - l;
        let pad = MOOV_IO_ACH_STRINGZEROS.get(&(m as i32)).unwrap();
        format!("{}{}", pad, s)
    }
}

fn populate_map(max: i32, zero: String) -> HashMap<i32, String> {
    let mut out = HashMap::new();

    for i in 0..max {
        out.insert(i, zero.repeat(i as usize));
    }

    out
}

