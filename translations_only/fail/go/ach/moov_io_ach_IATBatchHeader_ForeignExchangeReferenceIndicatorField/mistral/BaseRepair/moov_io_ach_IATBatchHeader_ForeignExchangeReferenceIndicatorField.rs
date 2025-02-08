

use std::collections::HashMap;
use std::fmt;

const MAX: usize = 94;

struct MoovIoAchConverters;

impl MoovIoAchConverters {
    fn numeric_field(&self, n: i32, max: u32) -> String {
        let s = n.to_string();
        let l = s.len() as u32;
        if l > max {
            return s[(l - max) as usize..].to_string();
        }
        let m = (max - l) as usize;
        let pad = get_pad(m);
        pad[..m].to_string() + &s
    }
}

fn get_pad(n: usize) -> String {
    let mut map = HashMap::new();
    for i in 0..=MAX {
        map.insert(i, "0".repeat(i));
    }
    map[&n].clone()
}

struct MoovIoAchIatBatchHeader {
    foreign_exchange_reference_indicator: i32,
}

impl MoovIoAchIatBatchHeader {
    fn numeric_field(&self, max: u32) -> String {
        MoovIoAchConverters.numeric_field(self.foreign_exchange_reference_indicator, max)
    }
}

impl fmt::Display for MoovIoAchIatBatchHeader {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "ForeignExchangeReferenceIndicator: {}"
            , self.numeric_field(1)
        )
    }
}

fn main() {
    let iat_batch_header = MoovIoAchIatBatchHeader {
        foreign_exchange_reference_indicator: 123,
    };
    println!("{}", iat_batch_header);
}

