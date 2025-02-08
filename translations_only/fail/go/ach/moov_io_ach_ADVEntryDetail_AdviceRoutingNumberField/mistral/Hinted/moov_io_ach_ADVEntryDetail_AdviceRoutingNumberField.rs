

use std::collections::HashMap;

const ZERO: &str = "0";

struct MoovIoAchAdvEntryDetail {
    advice_routing_number: String,
    // ... other fields elided ...
    moov_io_ach_converters: Box<MoovIoAchConverters>,
}

struct MoovIoAchConverters {
    moov_io_ach_string_zeros: Box<MoovIoAchStringZeros>,
}

impl MoovIoAchConverters {
    fn string_field(&self, s: &str, max: u32) -> String {
        let len = s.chars().count();
        if len > max as usize {
            return s[..(max as usize)].to_string();
        }

        let m = (max as usize - len) as usize;
        let pad = self.moov_io_ach_string_zeros.get(&m);
        if pad.is_some() {
            return pad.unwrap().to_string() + s;
        }

        "0".repeat(m) + s
    }
}

struct MoovIoAchStringZeros {
    data: HashMap<usize, String>,
}

impl MoovIoAchStringZeros {
    fn new(max: usize) -> MoovIoAchStringZeros {
        let mut data = HashMap::new();
        for i in 0..=max {
            data.insert(i, ZERO.repeat(i));
        }
        MoovIoAchStringZeros { data }
    }

    fn get(&self, n: &usize) -> Option<&String> {
        self.data.get(n)
    }
}

impl MoovIoAchAdvEntryDetail {
    fn advice_routing_number_field(&self) -> String {
        self.moov_io_ach_converters.string_field(&self.advice_routing_number, 9)
    }
}

fn main() {
    let moov_io_ach_string_zeros = Box::new(MoovIoAchStringZeros::new(94));
    let moov_io_ach_converters = Box::new(MoovIoAchConverters {
        moov_io_ach_string_zeros,
    });

    let mut ed = MoovIoAchAdvEntryDetail {
        advice_routing_number: "123456789".to_string(),
        moov_io_ach_converters,
    };

    println!("{}", ed.advice_routing_number_field());
}

