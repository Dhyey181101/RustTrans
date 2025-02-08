
use std::collections::HashMap;
use std::str::FromStr;
use once_cell::sync::Lazy;

static MOOV_IO_ACH_STRINGZEROS: Lazy<HashMap<i32, String>> = Lazy::new(|| populate_map(94, "0".to_string())); 

struct MoovIoAchAddenda16 {
    entry_detail_sequence_number: i32,
}

impl MoovIoAchAddenda16 {
    fn entry_detail_sequence_number_field(&self) -> String {
        numeric_field(self.entry_detail_sequence_number, 7)
    }
}

struct MoovIoAchConverters;

fn numeric_field(n: i32, max: u32) -> String {
    let s = n.to_string();
    if s.len() as u32 > max {
        s[s.len()-max as usize..].to_string()
    } else {
        let m = max - s.len() as u32;
        let pad = MOOV_IO_ACH_STRINGZEROS.get(&(m as i32)).unwrap();
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

