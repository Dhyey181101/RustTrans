
use std::collections::HashMap;
use std::fmt;
use once_cell::sync::Lazy;

static MOOV_IO_ACH_STRING_ZEROS: Lazy<HashMap<usize, String>> = Lazy::new(|| populate_map(94, "0".to_string()));

struct MoovIoAchAdvFileControl {
    batch_count: i32,
}

impl MoovIoAchAdvFileControl {
    fn batch_count_field(&self) -> String {
        numeric_field(self.batch_count, 6)
    }
}

struct MoovIoAchConverters;

fn numeric_field(n: i32, max: u32) -> String {
    let s = n.to_string();
    let l = s.len();

    if l > max as usize {
        s[l - max as usize..].to_string()
    } else {
        let m = (max - l as u32) as usize;
        match MOOV_IO_ACH_STRING_ZEROS.get(&m) {
            Some(pad) => format!("{}{}", pad, s),
            None => "0".repeat(m) + &s,
        }
    }
}

fn populate_map(max: usize, zero: String) -> HashMap<usize, String> {
    let mut out = HashMap::with_capacity(max);
    for i in 0..max {
        out.insert(i, zero.repeat(i));
    }
    out
}

