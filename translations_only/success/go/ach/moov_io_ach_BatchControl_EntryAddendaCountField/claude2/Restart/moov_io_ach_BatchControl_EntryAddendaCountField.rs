
use std::collections::HashMap;
use once_cell::sync::Lazy;

static MOOV_IO_ACH_STRINGZEROS: Lazy<HashMap<i32, String>> = Lazy::new(|| {
    moov_io_ach_populate_map(94, "0".to_string())
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
        s[(s.len() - max as usize)..].to_string()
    } else {
        let m = (max - s.len() as u32) as i32;
        MOOV_IO_ACH_STRINGZEROS.get(&m).unwrap().to_string() + &s
    }
}

fn moov_io_ach_populate_map(max: i32, zero: String) -> HashMap<i32, String> {
    let mut out = HashMap::new();
    for i in 0..max {
        out.insert(i, zero.repeat(i as usize));
    }
    out
}

struct MoovIoAchConverters;

