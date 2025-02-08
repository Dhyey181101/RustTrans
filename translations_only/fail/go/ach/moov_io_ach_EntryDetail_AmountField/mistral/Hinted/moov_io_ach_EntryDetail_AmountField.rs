

use std::fmt;
use std::collections::HashMap;
use std::boxed::Box;
use lazy_static::lazy_static;

struct MoovIoAchEntryDetail {
    amount: i32,
    // ... other fields elided ...
    converters: Box<MoovIoAchConverters>,
}

struct MoovIoAchConverters {
    max: usize,
    zeros: HashMap<usize, String>,
}

impl MoovIoAchEntryDetail {
    fn amount_field(&self) -> String {
        self.converters.numeric_field(self.amount, self.converters.max)
    }
}

impl MoovIoAchConverters {
    fn new(max: usize) -> Self {
        let mut zeros = HashMap::new();
        for i in 0..max {
            zeros.insert(i, "0".repeat(i));
        }
        MoovIoAchConverters { max, zeros }
    }

    fn numeric_field(&self, n: i32, max: usize) -> String {
        let s = n.to_string();
        let l = s.len() as usize;
        if l > max {
            s[l-max..].to_string()
        } else {
            let m = max - l;
            let pad = format!("{}", "0".repeat(m));
            format!("{}{}", pad, s)
        }
    }
}

const MOOV_IO_ACH_MAX: usize = 94;
lazy_static! {
    static ref MOOV_IO_ACH_STRING_ZEROS: HashMap<usize, String> = {
        let mut map = HashMap::new();
        for i in 0..MOOV_IO_ACH_MAX {
            map.insert(i, "0".repeat(i));
        }
        map
    };
}

impl fmt::Display for MoovIoAchEntryDetail {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // ... implementation elided ...
        write!(f, "Amount: {}", self.amount_field())
    }
}

