
use std::collections::HashMap;
use std::str::FromStr;
use once_cell::sync::Lazy;

static MOOV_IO_ACH_STRINGZEROS: Lazy<HashMap<u32, String>> = Lazy::new(|| populate_map(94, "0".to_string()));

struct MoovIoAchAdvFileControl {
    block_count: i32,
}

impl MoovIoAchAdvFileControl {
    fn block_count_field(&self) -> String {
        numeric_field(self.block_count, 6)
    }
}

struct MoovIoAchConverters;

fn numeric_field(n: i32, max: u32) -> String {
    let s = n.to_string();
    let l = s.len() as u32;
    
    if l > max {
        s[s.len() - max as usize..].to_string()
    } else {
        let m = max - l;
        let pad = MOOV_IO_ACH_STRINGZEROS.get(&m).unwrap().clone();
        format!("{}{}", pad, s)
    }
}

fn populate_map(max: i32, zero: String) -> HashMap<u32, String> {
    let mut map = HashMap::new();
    for i in 0..max {
        map.insert(i as u32, zero.repeat(i as usize));
    }
    map
}

