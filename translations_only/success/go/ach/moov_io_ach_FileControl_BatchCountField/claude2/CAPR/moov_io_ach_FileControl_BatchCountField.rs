
use std::collections::HashMap;
use std::fmt;
use once_cell::sync::Lazy;

static MOOV_IO_ACH_STRINGZEROS: Lazy<HashMap<i32, String>> = Lazy::new(|| HashMap::new());

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
    let l = s.len() as u32;
    if l > max {
        let idx = l-max;
        let range = idx..l;
        s.chars().skip(idx as usize).collect()
    } else {
        let m = max - l;
        let pad = "0".repeat(m as usize);
        format!("{}{}", pad, s)
    }
}

