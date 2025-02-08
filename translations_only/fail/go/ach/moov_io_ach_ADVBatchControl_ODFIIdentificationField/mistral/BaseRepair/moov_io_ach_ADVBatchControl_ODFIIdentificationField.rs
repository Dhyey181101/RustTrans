

use std::collections::HashMap;
use std::fmt;

struct MoovIoAchConverters {
    zero_data: Box<MoovIoAchStringZeros>,
}

impl MoovIoAchConverters {
    fn string_field(&self, s: &str, max: usize) -> String {
        let ln = s.len();
        if ln > max {
            s[..max].to_string()
        } else {
            let m = max - ln;
            let pad = self.zero_data.get(&m).map(|s| s.clone());
            if pad.is_some() {
                format!("{}{}", pad.unwrap(), s)
            } else {
                "0".repeat(m) + s
            }
        }
    }
}

struct MoovIoAchStringZeros {
    data: HashMap<usize, String>,
}

impl MoovIoAchStringZeros {
    fn new(max: usize) -> MoovIoAchStringZeros {
        let mut data = HashMap::new();
        for i in 0..=max {
            data.insert(i, "0".repeat(i));
        }
        MoovIoAchStringZeros { data }
    }

    fn get(&self, size: &usize) -> Option<&String> {
        self.data.get(size)
    }
}

struct MoovIoAchAdvBatchControl {
    odfi_identification: String,
    // ... other fields ...
    moov_io_ach_converters: MoovIoAchConverters,
}

impl MoovIoAchAdvBatchControl {
    fn odfi_identification_field(&self) -> String {
        self.odfi_identification.clone()
    }
}

fn main() {}

