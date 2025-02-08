

use std::collections::HashMap;
use std::fmt;
use std::fmt::Display;
use std::iter;
use std::str;

struct MoovIoAchFileControl {
    batch_count: i32,
    _private_field: i32,
}

struct MoovIoAchConverters;

impl MoovIoAchConverters {
    fn numeric_field(&self, n: i32, max: u32) -> String {
        let s = n.to_string();
        let l = s.len() as u32;
        if l > max {
            return s[(l - max) as usize..].to_string();
        } else {
            let m = max - l;
            let pad = get_pad_string(m as u32);
            return pad.repeat(m as usize) + &s;
        }
    }
}

fn get_pad_string(n: u32) -> String {
    "0".repeat(n as usize)
}

fn moov_io_ach_populate_map(max: i32, zero: char) -> HashMap<i32, String> {
    let mut out = HashMap::new();
    for i in 0..max {
        out.insert(i, get_pad_string(i as u32));
    }
    out
}

impl Display for MoovIoAchFileControl {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let converters = MoovIoAchConverters;
        write!(f, "{}", converters.numeric_field(self.batch_count, 6))
    }
}

fn main() {
    let file_control_1 = MoovIoAchFileControl {
        batch_count: 168,
        _private_field: 1611292681,
    };
    let file_control_2 = MoovIoAchFileControl {
        batch_count: 36960,
        _private_field: -1627455489,
    };
    let file_control_3 = MoovIoAchFileControl {
        batch_count: 1612840800,
        _private_field: 9490176,
    };
    let file_control_4 = MoovIoAchFileControl {
        batch_count: -1077152,
        _private_field: -141533183,
    };

    println!("{}", file_control_1);
    println!("{}", file_control_2);
    println!("{}", file_control_3);
    println!("{}", file_control_4);
}

