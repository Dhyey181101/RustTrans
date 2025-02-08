
use std::collections::HashMap;
use once_cell::sync::Lazy;

static MOOV_IO_ACH_STRINGZEROS: Lazy<HashMap<i32, String>> = Lazy::new(|| {
    moov_io_ach_populate_map(94, "0")
});

struct MoovIoAchBatchControl {
    entry_addenda_count: i32,
}

impl MoovIoAchBatchControl {
    fn entry_addenda_count_field(&self) -> String {
        MoovIoAchConverters::numeric_field(self.entry_addenda_count, 6)
    }
}

struct MoovIoAchConverters;

impl MoovIoAchConverters {
    fn numeric_field(n: i32, max: u32) -> String {
        let s = n.to_string();
        if s.len() as u32 > max {
            s[s.len()-max as usize..].to_string()
        } else {
            let m = (max - s.len() as u32) as i32;
            let pad = MOOV_IO_ACH_STRINGZEROS.get(&m).unwrap().to_string();
            pad + &s
        }
    }
}

fn moov_io_ach_populate_map(max: i32, zero: &str) -> HashMap<i32, String> {
    let mut out = HashMap::with_capacity(max as usize);
    for i in 0..max {
        out.insert(i, zero.repeat(i as usize));
    }
    out
}

