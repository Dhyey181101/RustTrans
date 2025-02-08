
use std::collections::HashMap;

lazy_static! {
    static ref MOOV_IO_ACH_STRING_ZEROS: HashMap<usize, String> = moov_io_ach_populate_map(94, "0");
}

fn moov_io_ach_populate_map(max: usize, zero: &str) -> HashMap<usize, String> {
    let mut out = HashMap::with_capacity(max);
    for i in 0..max {
        out.insert(i, zero.repeat(i));
    }
    out
}

struct MoovIoAchADVBatchControl {
    entry_addenda_count: i32,
}

impl MoovIoAchADVBatchControl {
    fn entry_addenda_count_field(&self) -> String {
        MoovIoAchConverters::numeric_field(self.entry_addenda_count, 6)
    }
}

struct MoovIoAchConverters;

impl MoovIoAchConverters {
    fn numeric_field(n: i32, max: usize) -> String {
        let s = n.to_string();
        let l = s.len();
        if l > max {
            s[l - max..].to_string()
        } else {
            let m = max - l;
            if let Some(pad) = MOOV_IO_ACH_STRING_ZEROS.get(&m) {
                format!("{}{}", pad, s)
            } else {
                // slow path
                "0".repeat(m) + &s
            }
        }
    }
}

impl MoovIoAchADVBatchControl {
    fn new(entry_addenda_count: i32) -> Box<Self> {
        Box::new(Self {
            entry_addenda_count,
            ..Default::default()
        })
    }
}

impl Default for MoovIoAchADVBatchControl {
    fn default() -> Self {
        Self {
            entry_addenda_count: 0,
        }
    }
}

impl Default for MoovIoAchConverters {
    fn default() -> Self {
        Self {}
    }
}

#[macro_use]
extern crate lazy_static;
