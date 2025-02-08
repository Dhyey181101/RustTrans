

use std::collections::HashMap;
use std::fmt;

const ZEROS: &str = "0";

struct MoovIoAchIatBatchHeader {
    odfi_identification: String,
    converters: MoovIoAchConverters,
}

impl MoovIoAchIatBatchHeader {
    fn odfi_identification_field(&self) -> String {
        self.odfi_identification.clone()
    }
}

struct MoovIoAchConverters;

impl MoovIoAchConverters {
    fn string_field(&self, s: &String, max: u32) -> String {
        let ln = s.chars().count() as u32;
        if ln > max {
            s[..(max as usize)].to_string()
        } else {
            let m = (max - ln) as usize;
            let pad = get_zeros(m);
            pad + s
        }
    }
}

fn get_zeros(n: usize) -> String {
    let mut zeros = String::from(ZEROS);
    for _ in 0..n - 1 {
        zeros.push_str(ZEROS);
    }
    zeros
}

fn main() {
    let mut map = HashMap::new();
    for i in 0..100 {
        map.insert(i, get_zeros(i as usize));
    }
}

impl fmt::Display for MoovIoAchIatBatchHeader {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "MoovIoAchIatBatchHeader {{
                odfi_identification: {},
                converters: {}
            }}",
            self.odfi_identification, self.converters
        )
    }
}

impl fmt::Display for MoovIoAchConverters {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "MoovIoAchConverters {{}}")
    }
}

