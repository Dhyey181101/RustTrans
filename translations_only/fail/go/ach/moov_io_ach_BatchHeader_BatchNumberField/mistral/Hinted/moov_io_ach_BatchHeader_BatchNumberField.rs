

use std::collections::HashMap;
use std::fmt;
use std::string::String;

struct MoovIoAchBatchHeader {
    batch_number: i32,
    _converters: MoovIoAchConverters,
}

struct MoovIoAchConverters;

impl MoovIoAchBatchHeader {
    fn batch_number_field(&self) -> String {
        self._converters.numeric_field(self.batch_number, 7)
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
            let pad = self.get_pad_string(m);
            return pad + &s;
        }
    }

    fn get_pad_string(&self, n: u32) -> String {
        let mut out = HashMap::new();
        for i in 0..=n {
            out.insert(i, "0".repeat(i as usize));
        }
        out[&n].clone()
    }
}

impl fmt::Display for MoovIoAchBatchHeader {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.batch_number_field())
    }
}

fn main() {
    let batch_header = MoovIoAchBatchHeader {
        batch_number: -64257,
        _converters: MoovIoAchConverters {},
    };
    println!("{}", batch_header);

    let batch_header = MoovIoAchBatchHeader {
        batch_number: 605,
        _converters: MoovIoAchConverters {},
    };
    println!("{}", batch_header);

    let batch_header = MoovIoAchBatchHeader {
        batch_number: 1982398464,
        _converters: MoovIoAchConverters {},
    };
    println!("{}", batch_header);

    let batch_header = MoovIoAchBatchHeader {
        batch_number: 570425186,
        _converters: MoovIoAchConverters {},
    };
    println!("{}", batch_header);
}

