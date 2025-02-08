
use std::collections::HashMap;
use std::convert::From;
use std::fmt;
use std::string::ToString;

use once_cell::sync::Lazy;

static MOOV_IO_ACH_STRING_ZEROS: Lazy<HashMap<i32, String>> = Lazy::new(|| {
    populate_map(94, "0".to_string())
});

struct MoovIoAchFileControl {
    batch_count: i32,
}

impl MoovIoAchFileControl {
    fn batch_count_field(&self) -> String {
        numeric_field(self.batch_count, 6)
    }
}

struct MoovIoAchConverters;

fn numeric_field(n: i32, max: u32) -> String {
    let s = n.to_string();
    if s.len() as u32 > max {
        s[s.len() - max as usize..].to_string()
    } else {
        let m = max - s.len() as u32;
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

