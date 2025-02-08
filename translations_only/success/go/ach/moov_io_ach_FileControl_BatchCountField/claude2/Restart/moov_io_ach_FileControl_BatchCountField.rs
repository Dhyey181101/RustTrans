
use std::collections::HashMap;
use std::fmt::Write;
use once_cell::sync::Lazy;

static MOOV_IO_ACH_STRING_ZEROS: Lazy<HashMap<i32, String>> = Lazy::new(|| populate_map(94, "0".to_string()));

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
    let mut s = n.to_string();
    if s.len() as u32 > max {
        s.drain(..s.len() - max as usize);
        s
    } else {
        let mut pad = String::with_capacity(max as usize);
        for _ in 0..(max - s.len() as u32) {
            write!(&mut pad, "0").unwrap();
        }
        pad + &s
    }
}

fn populate_map(max: i32, zero: String) -> HashMap<i32, String> {
    let mut out = HashMap::with_capacity(max as usize);
    for i in 0..max {
        out.insert(i, zero.repeat(i as usize));
    }
    out
}

