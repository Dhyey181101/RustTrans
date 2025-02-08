

use std::collections::HashMap;
use std::fmt;
use std::str;

const ZEROS: &str = "0";

struct MoovIoAchAdvBatchControl {
    total_credit: i32,
    moov_io_ach_converters: Box<MoovIoAchConverters>,
}

struct MoovIoAchConverters {
    pad_string: HashMap<usize, String>,
}

impl MoovIoAchAdvBatchControl {
    fn total_credit_entry_dollar_amount_field(&self) -> String {
        self.moov_io_ach_converters.numeric_field(self.total_credit, 20)
    }
}

impl MoovIoAchConverters {
    fn new() -> MoovIoAchConverters {
        MoovIoAchConverters {
            pad_string: get_pad_string(),
        }
    }

    fn numeric_field(&self, n: i32, max: u32) -> String {
        let s = n.to_string();
        let l = s.len() as u32;
        if l > max {
            return s[(l - max) as usize..].to_string();
        } else {
            let m = max - l;
            let pad = self.pad_string.get(&(m as usize)).unwrap();
            return pad.clone() + &s;
        }
    }
}

fn get_pad_string() -> HashMap<usize, String> {
    let mut out = HashMap::new();
    for i in 0..=20 {
        out.insert(i, "0".repeat(i));
    }
    out
}

impl fmt::Display for MoovIoAchAdvBatchControl {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{{total_credit:{}}}",
            self.total_credit_entry_dollar_amount_field()
        )
    }
}

fn main() {
    let bc = MoovIoAchAdvBatchControl {
        total_credit: 123456,
        moov_io_ach_converters: Box::new(MoovIoAchConverters::new()),
    };
    println!("{}", bc);
}

