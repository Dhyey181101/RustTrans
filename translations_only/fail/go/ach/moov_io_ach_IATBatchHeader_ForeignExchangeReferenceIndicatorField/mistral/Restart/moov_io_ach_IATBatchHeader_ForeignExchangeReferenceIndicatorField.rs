

use std::collections::HashMap;
use std::fmt;
use std::str;

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
    for i in 0..=n {
        map.insert(i, "0".repeat(i));
    }
    map[&n].clone()
}

struct MoovIoAchIatBatchHeader {
    foreign_exchange_reference_indicator: i32,
    // ... other fields
    moov_io_ach_converters: Box<MoovIoAchConverters>,
}

impl MoovIoAchIatBatchHeader {
    fn foreign_exchange_reference_indicator(&self) -> String {
        self.moov_io_ach_converters.numeric_field(self.foreign_exchange_reference_indicator, 1)
    }
}

impl fmt::Display for MoovIoAchIatBatchHeader {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "ForeignExchangeReferenceIndicator: {:?}",
            self.foreign_exchange_reference_indicator()
        )
    }
}

fn main() {
    let iat_batch_header = MoovIoAchIatBatchHeader {
        foreign_exchange_reference_indicator: 123,
        // ... other fields
        moov_io_ach_converters: Box::new(MoovIoAchConverters),
    };
    println!("{}", iat_batch_header);
}

