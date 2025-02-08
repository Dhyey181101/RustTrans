

use std::collections::HashMap;
use once_cell::sync::Lazy;

static MOOV_IO_ACH_STRING_ZEROS: Lazy<HashMap<i32, String>> = Lazy::new(|| populate_map(94, "0".to_string()));

struct MoovIoAchFileControl {
    block_count: i32,
}

struct MoovIoAchConverters;

impl MoovIoAchFileControl {
    fn block_count_field(&self) -> String {
        numeric_field(self.block_count, 6)
    }
}

fn numeric_field(n: i32, max: u32) -> String {
    let s = n.to_string();
    let l = s.len() as u32;

    if l > max {
        s[..max as usize].to_string()
    } else {
        let m = max - l;
        let pad = MOOV_IO_ACH_STRING_ZEROS.get(&(m as i32)).unwrap();
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


