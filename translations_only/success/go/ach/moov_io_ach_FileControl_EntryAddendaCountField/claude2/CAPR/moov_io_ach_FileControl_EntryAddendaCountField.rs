
use std::collections::HashMap;
use once_cell::sync::Lazy;

static MOOV_IO_ACH_STRING_ZEROS: Lazy<HashMap<u32, String>> = Lazy::new(|| {
    populate_map(94, "0".to_string())  
});

struct MoovIoAchFileControl {
    entry_addenda_count: i32,
}

impl MoovIoAchFileControl {
    fn entry_addenda_count_field(&self) -> String {
        numeric_field(self.entry_addenda_count, 8)
    }
}

struct MoovIoAchConverters;

fn numeric_field(n: i32, max: u32) -> String {
    let s = n.to_string();
    if s.len() as u32 > max {
        s[s.len() - max as usize..].to_string()
    } else {
        let m = max - s.len() as u32;
        MOOV_IO_ACH_STRING_ZEROS.get(&m).unwrap().clone() + &s
    }
}

fn populate_map(max: i32, zero: String) -> HashMap<u32, String> {
    let mut out = HashMap::new();
    for i in 0..max {
        out.insert(i as u32, zero.repeat(i as usize));
    }
    out
}

