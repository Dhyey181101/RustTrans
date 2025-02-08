

use std::fmt;
use std::collections::HashMap;
use std::str;
use std::iter;

struct MoovIoAchConverters;

impl MoovIoAchConverters {
    fn numeric_field(&self, n: i32, max: usize) -> String {
        let s = n.to_string();
        let l = s.len();
        if l > max {
            s[l-max..].to_string()
        } else {
            let m = max - l;
            let pad = get_pad_string(m);
            pad + &s
        }
    }
}

fn get_pad_string(n: usize) -> String {
    iter::repeat("0").take(n).collect::<String>()
}

struct MoovIoAchAdvfilecontrol {
    entry_hash: i32,
    moov_io_ach_converters: Box<MoovIoAchConverters>,
}

impl MoovIoAchAdvfilecontrol {
    fn entry_hash_field(&self) -> String {
        self.moov_io_ach_converters.numeric_field(self.entry_hash, 10)
    }
}

struct MoovIoAchFilecontrol {
    entry_hash: i32,
    moov_io_ach_converters: Box<MoovIoAchConverters>,
}

impl MoovIoAchFilecontrol {
    fn new() -> MoovIoAchFilecontrol {
        MoovIoAchFilecontrol {
            entry_hash: 0,
            moov_io_ach_converters: Box::new(MoovIoAchConverters),
        }
    }

    fn entry_hash_field(&self) -> String {
        self.moov_io_ach_converters.numeric_field(self.entry_hash, 10)
    }
}

fn main() {
    let mut file_control = MoovIoAchFilecontrol::new();
    println!("{}", file_control.entry_hash_field());
}

