

use std::collections::HashMap;
use std::fmt;
use std::str;

const ZEROS: &str = "0";

struct MoovIoAchAdvBatchControl {
    entry_addenda_count: i32,
    // ... other fields elided ...
    moov_io_ach_converters: Box<MoovIoAchConverters>,
}

struct MoovIoAchConverters {
    _priv: (),
}

impl MoovIoAchAdvBatchControl {
    fn entry_addenda_count_field(&self) -> String {
        self.moov_io_ach_converters.numeric_field(self.entry_addenda_count, 6)
    }
}

impl MoovIoAchConverters {
    fn numeric_field(&self, n: i32, max: u32) -> String {
        let s = n.to_string();
        let l = s.len() as u32;
        if l > max {
            s[(l - max) as usize..].to_string()
        } else {
            let m = max - l;
            let pad = get_pad(m as usize);
            pad + &s
        }
    }
}

fn get_pad(n: usize) -> String {
    let mut out = HashMap::new();
    for i in 0..100 {
        out.insert(i, "0".repeat(i));
    }
    out[&n].clone()
}

impl fmt::Display for MoovIoAchAdvBatchControl {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // ... formatting elided ...
        write!(
            f,
            "EntryAddendaCount:{},",
            self.entry_addenda_count_field()
        )
    }
}

