
use std::collections::HashMap;

lazy_static::lazy_static! {
    static ref MOOV_IO_ACH_STRING_ZEROS: HashMap<i32, String> = moov_io_ach_populate_map(94, "0");
}

fn moov_io_ach_populate_map(max: i32, zero: &str) -> HashMap<i32, String> {
    let mut out = HashMap::with_capacity(max as usize);
    for i in 0..max {
        out.insert(i, zero.repeat(i as usize));
    }
    out
}

struct MoovIoAchADVFileControl {
    entry_addenda_count: i32,
}

impl MoovIoAchADVFileControl {
    fn entry_addenda_count_field(&self) -> String {
        MoovIoAchConverters::numeric_field(self.entry_addenda_count, 8)
    }

    fn new(entry_addenda_count: i32) -> Box<Self> {
        Box::new(Self {
            entry_addenda_count,
        })
    }
}

struct MoovIoAchConverters;

impl MoovIoAchConverters {
    fn numeric_field(n: i32, max: u32) -> String {
        let s = n.to_string();
        let l = s.len() as u32;
        if l > max {
            s.chars().skip((l - max) as usize).collect()
        } else {
            let m = (max - l) as i32;
            if let Some(pad) = MOOV_IO_ACH_STRING_ZEROS.get(&m) {
                format!("{}{}", pad, s)
            } else {
                // slow path
                "0".repeat(m as usize) + &s
            }
        }
    }
}
