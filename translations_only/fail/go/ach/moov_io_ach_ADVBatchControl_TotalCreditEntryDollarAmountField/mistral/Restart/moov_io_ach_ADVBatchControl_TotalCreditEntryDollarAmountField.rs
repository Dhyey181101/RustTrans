

use std::collections::HashMap;
use std::fmt;
use std::str;

const ZEROS: &str = "0";

struct MoovIoAchAdvBatchControl {
    total_credit: i32,
    moov_io_ach_converters: Box<MoovIoAchConverters>,
}

struct MoovIoAchConverters {
    numeric_field_pad: HashMap<usize, String>,
}

impl MoovIoAchConverters {
    fn new() -> MoovIoAchConverters {
        let mut out = HashMap::new();
        for i in 0..=94 {
            out.insert(i, "0".repeat(i));
        }
        MoovIoAchConverters {
            numeric_field_pad: out,
        }
    }

    fn numeric_field(&self, n: i32, max: u32) -> String {
        let s = n.to_string();
        let l = s.len() as u32;
        if l > max {
            return s[(l - max) as usize..].to_string();
        } else {
            let m = max - l;
            let pad = &self.numeric_field_pad[&(m as usize)];
            return pad.to_string() + &s;
        }
    }
}

impl fmt::Display for MoovIoAchAdvBatchControl {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{{total_credit:{}}}",
            self.moov_io_ach_converters.numeric_field(self.total_credit, 20)
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

