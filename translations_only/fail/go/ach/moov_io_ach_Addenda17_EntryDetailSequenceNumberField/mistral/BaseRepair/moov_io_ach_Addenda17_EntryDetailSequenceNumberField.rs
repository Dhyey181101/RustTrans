

use std::fmt;
use std::collections::HashMap;

struct MoovIoAchConverters;

impl MoovIoAchConverters {
    fn numeric_field(&self, n: i32, max: u32) -> String {
        let s = n.to_string();
        let l = s.len() as u32;
        if l > max {
            s[(l - max) as usize..].to_string()
        } else {
            let m = max - l;
            let pad = MoovIoAchPopulateMap::get_pad(m as usize);
            pad + &s
        }
    }
}

struct MoovIoAchPopulateMap;

impl MoovIoAchPopulateMap {
    fn get_pad(num: usize) -> String {
        let mut map: HashMap<usize, String> = HashMap::new();
        for i in 0..94 {
            map.insert(i, "0".repeat(i));
        }
        let binding = "0".repeat(num);
        let pad = map.get(&num).unwrap_or(&&binding);
        pad.clone()
    }
}

struct MoovIoAchAddenda17 {
    entry_detail_sequence_number: i32,
    moov_io_ach_converters: Box<MoovIoAchConverters>,
}

impl MoovIoAchAddenda17 {
    fn entry_detail_sequence_number_field(&self) -> String {
        self.numeric_field(self.entry_detail_sequence_number, 7)
    }

    fn numeric_field(&self, n: i32, max: u32) -> String {
        self.moov_io_ach_converters.numeric_field(n, max)
    }
}

impl fmt::Display for MoovIoAchAddenda17 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{ entryDetailSequenceNumber: {} }}", self.entry_detail_sequence_number)
    }
}

