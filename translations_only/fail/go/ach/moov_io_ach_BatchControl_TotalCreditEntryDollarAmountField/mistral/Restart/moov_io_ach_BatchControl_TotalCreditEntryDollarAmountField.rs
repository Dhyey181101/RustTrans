

use std::collections::HashMap;
use std::fmt;
use std::boxed::Box;

const MAX: usize = 94;
const ZERO: &str = "0";

struct MoovIoAchBatchControl {
    total_credit: i32,
    converters: MoovIoAchConverters,
}

struct MoovIoAchConverters {
    numeric_field: Box<dyn Fn(&MoovIoAchConverters, i32, u32) -> String>,
}

impl MoovIoAchBatchControl {
    fn total_credit_entry_dollar_amount_field(&self) -> String {
        (self.converters.numeric_field)(
            &self.converters,
            self.total_credit,
            12,
        )
    }
}

impl MoovIoAchConverters {
    fn numeric_field(&self, n: i32, max: u32) -> String {
        let s = n.to_string();
        let l = s.len() as u32;
        if l > max {
            return s[(l - max) as usize..].to_string();
        } else {
            let m = max - l;
            let pad = get_pad(m as usize);
            return pad + &s;
        }
    }
}

fn get_pad(n: usize) -> String {
    let mut out = HashMap::new();
    for i in 0..=n {
        out.insert(i, "0".repeat(i));
    }
    out[&n].clone()
}

impl fmt::Display for MoovIoAchBatchControl {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "total_credit: {}, converters: {{}}",
            self.total_credit_entry_dollar_amount_field()
        )
    }
}

fn main() {
    let bc = MoovIoAchBatchControl {
        total_credit: 123456,
        converters: MoovIoAchConverters {
            numeric_field: Box::new(MoovIoAchConverters::numeric_field),
        },
    };
    println!("{}", bc);
}

