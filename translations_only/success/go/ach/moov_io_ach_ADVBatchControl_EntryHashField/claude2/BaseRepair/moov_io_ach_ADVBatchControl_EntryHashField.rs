
use std::collections::HashMap;
use std::str::FromStr;
use once_cell::sync::Lazy;

static MOV_IO_ACH_STRINGZEROS: Lazy<HashMap<i32, String>> = Lazy::new(|| populate_map(94, "0".to_string()));

struct MoovIoAchAdBatchControl {
    entry_hash: i32,
}

impl MoovIoAchAdBatchControl {
    fn entry_hash_field(&self) -> String {
        numeric_field(self.entry_hash, 10)
    }
}

fn numeric_field(n: i32, max: u32) -> String {
    let s = n.to_string();
    let l = s.len();

    if l as u32 > max {
        s[l - max as usize..].to_string()
    } else {
        let m = max - l as u32;
        let pad = MOV_IO_ACH_STRINGZEROS.get(&(m as i32)).unwrap();
        format!("{}{}", pad, s)
    }
}

fn populate_map(max: i32, zero: String) -> HashMap<i32, String> {
    let mut map = HashMap::new();
    for i in 0..max {
        map.insert(i, zero.repeat(i as usize));
    }
    map
}

struct MoovIoAchConverters;

