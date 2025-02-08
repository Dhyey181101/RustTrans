

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
    numeric_field_pad: HashMap<usize, String>,
}

impl MoovIoAchConverters {
    fn new() -> Self {
        MoovIoAchConverters {
            numeric_field_pad: get_pad_string(10),
        }
    }

    fn numeric_field(&self, n: i32, max: u32) -> String {
        let s = n.to_string();
        let l = s.len() as u32;
        if l > max {
            let start = l - max;
            let mut result = String::new();
            for (i, c) in s.chars().enumerate() {
                if i >= start as usize {
                    result.push(c);
                }
            }
            result
        } else {
            let m = max - l;
            self.numeric_field_pad[&(m as usize)].clone() + &s
        }
    }
}

fn get_pad_string(n: usize) -> HashMap<usize, String> {
    let mut out = HashMap::new();
    for i in 0..=n {
        out.insert(i, "0".repeat(i));
    }
    out
}

impl MoovIoAchAdvBatchControl {
    fn entry_addenda_count_field(&self) -> String {
        self.moov_io_ach_converters.numeric_field(self.entry_addenda_count, 6)
    }
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

