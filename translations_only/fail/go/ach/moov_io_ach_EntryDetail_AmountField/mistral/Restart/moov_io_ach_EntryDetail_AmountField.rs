

use std::fmt;
use std::collections::HashMap;

struct MoovIoAchEntryDetail {
    amount: i32,
    // ... other fields omitted for brevity
    converters: Box<MoovIoAchConverters>,
}

struct MoovIoAchConverters {
    pad_strings: HashMap<u32, String>,
}

impl MoovIoAchConverters {
    fn new() -> Self {
        let mut map = HashMap::new();
        populate_map(&mut map, 94, "0");
        MoovIoAchConverters { pad_strings: map }
    }

    fn numeric_field(&self, n: i32, max: u32) -> String {
        let s = n.to_string();
        let l = s.len();
        if l > max as usize {
            s[l-max as usize..].to_string()
        } else {
            let m = max - l as u32;
            let pad_string = self.pad_strings.get(&m).unwrap_or(&"0".repeat(m as usize)).clone();
            pad_string
        }
    }
}

fn populate_map(map: &mut HashMap<u32, String>, max: u32, zero: &str) {
    for i in 0..max {
        let mut s = String::new();
        s.push_str(zero);
        for _ in 0..i {
            s.push_str(zero);
        }
        map.insert(i, s);
    }
}

impl fmt::Display for MoovIoAchEntryDetail {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", "Amount: ", self.converters.numeric_field(self.amount, 10))
    }
}

fn main() {
    let ed = MoovIoAchEntryDetail {
        amount: 12345,
        // ... other fields initialization omitted for brevity
        converters: Box::new(MoovIoAchConverters::new()),
    };
    println!("{}", ed);
}

