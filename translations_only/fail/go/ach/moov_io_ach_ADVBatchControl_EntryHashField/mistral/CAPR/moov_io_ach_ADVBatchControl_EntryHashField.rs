

use std::collections::HashMap;
use std::fmt;
use std::str;

const ZEROS: &str = "0";

struct MoovIoAchAdvBatchControl {
    entry_hash: i32,
    // ... other fields ...
    moov_io_ach_converters: Box<MoovIoAchConverters>,
}

struct MoovIoAchConverters {
    numeric_field_max_len: u32,
}

impl MoovIoAchAdvBatchControl {
    fn entry_hash_field(&self) -> String {
        self.moov_io_ach_converters.numeric_field(self.entry_hash, self.moov_io_ach_converters.numeric_field_max_len)
    }
}

impl MoovIoAchConverters {
    fn numeric_field(&self, n: i32, max: u32) -> String {
        let s = n.to_string();
        let l = s.len() as u32;
        if l > max {
            let zeros_slice = ZEROS.as_bytes();
            let mut padded_s = String::from("");
            for i in 0..(max - l) {
                padded_s.push((zeros_slice[i as usize] as char));
            }
            padded_s += &s;
            padded_s
        } else {
            s
        }
    }
}

