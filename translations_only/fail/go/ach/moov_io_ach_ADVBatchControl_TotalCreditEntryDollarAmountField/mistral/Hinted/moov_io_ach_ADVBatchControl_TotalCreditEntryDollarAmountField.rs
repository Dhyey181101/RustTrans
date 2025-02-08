

use std::collections::HashMap;
use std::fmt;

const ZEROS: &str = "0";

#[derive(Debug)]
struct MoovIoAchConverters;

impl MoovIoAchConverters {
    fn numeric_field(&self, n: i32, max: u32) -> String {
        let s = n.to_string();
        let l = s.len() as u32;
        if l > max {
            return s[(l - max) as usize..].to_string();
        }
        let m = max - l;
        let pad = get_pad(m as usize);
        pad.chars().rev().chain(s.chars()).collect::<String>()
    }
}

fn get_pad(n: usize) -> String {
    let mut out = HashMap::new();
    for i in 0..=n {
        out.insert(i, "0".repeat(i));
    }
    out[&n].to_string()
}

struct MoovIoAchAdvBatchControl {
    total_credit: i32,
    converters: Box<MoovIoAchConverters>,
}

impl MoovIoAchAdvBatchControl {
    fn total_credit_entry_dollar_amount_field(&self) -> String {
        self.converters.numeric_field(self.total_credit, 20)
    }
}

impl fmt::Display for MoovIoAchAdvBatchControl {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{{ total_credit: {}, converters: BoxNew {{ inner: {:?} }} }}",
            self.total_credit,
            *self.converters
        )
    }
}

fn main() {
    let batch_control = MoovIoAchAdvBatchControl {
        total_credit: 2005535242,
        converters: Box::new(MoovIoAchConverters),
    };
    println!(
        "{}",
        batch_control.total_credit_entry_dollar_amount_field()
    );
}

