

use std::fmt;
use std::collections::HashMap;
use std::convert::TryInto;

struct MoovIoAchConverters;

impl MoovIoAchConverters {
    fn numeric_field(&self, n: i32, max: u32) -> String {
        let s = n.to_string();
        let l = s.len() as u32;
        if l > max {
            return s[(l - max) as usize..].to_string();
        } else {
            let m = max - l;
            let pad = MoovIoAchPopulateMap::get_pad(m as usize);
            return format!("{}{}", pad, s);
        }
    }
}

struct MoovIoAchPopulateMap;

impl MoovIoAchPopulateMap {
    fn get_pad(num: usize) -> String {
        "0".repeat(num)
    }
}

struct MoovIoAchAddenda16 {
    entry_detail_sequence_number: i32,
    moov_io_ach_converters: Box<MoovIoAchConverters>,
}

impl MoovIoAchAddenda16 {
    fn entry_detail_sequence_number_field(&self) -> String {
        self.moov_io_ach_converters.numeric_field(self.entry_detail_sequence_number, 7)
    }
}

impl fmt::Display for MoovIoAchAddenda16 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "MoovIoAchAddenda16 [entry_detail_sequence_number={}]", self.entry_detail_sequence_number)
    }
}

