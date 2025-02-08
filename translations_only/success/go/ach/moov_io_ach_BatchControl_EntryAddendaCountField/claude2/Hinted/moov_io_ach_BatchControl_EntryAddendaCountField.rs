
use std::collections::HashMap;
use once_cell::sync::Lazy;

static MOOV_IO_ACH_STRINGZEROS: Lazy<HashMap<i32, String>> = Lazy::new(|| {
    let mut m = HashMap::new();
    for i in 0..94 {
        m.insert(i, "0".repeat(i as usize));
    }
    m
});

struct MoovIoAchBatchControl {
    entry_addenda_count: i32,
}

impl MoovIoAchBatchControl {
    fn entry_addenda_count_field(&self) -> String {
        numeric_field(self.entry_addenda_count, 6)
    }
}

fn numeric_field(n: i32, max: u32) -> String {
    let s = n.to_string();
    if s.len() as u32 > max {
        s[s.len() - max as usize..].to_string()
    } else {
        let m = (max - s.len() as u32) as i32;
        MOOV_IO_ACH_STRINGZEROS.get(&m).unwrap().to_string() + &s
    }
}

struct MoovIoAchConverters;
