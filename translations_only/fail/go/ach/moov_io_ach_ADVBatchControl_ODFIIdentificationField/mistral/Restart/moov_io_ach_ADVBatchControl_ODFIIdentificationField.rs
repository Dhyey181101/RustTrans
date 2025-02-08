

use std::collections::HashMap;
use std::fmt;

struct MoovIoAchConverters;

impl MoovIoAchConverters {
    fn string_field(&self, s: &str, max: usize, string_zeros: &MoovIoAchStringZeros) -> String {
        let ln = s.len();
        if ln > max {
            return s[..max].to_string();
        }

        let m = max - ln;
        let pad = MoovIoAchStringZeros::get(string_zeros, &m);
        if pad.is_some() {
            return format!("{}{}", pad.unwrap(), s);
        }

        "0".repeat(m) + s
    }
}

struct MoovIoAchStringZeros {
    data: HashMap<usize, String>,
}

impl MoovIoAchStringZeros {
    fn new(max: usize) -> Self {
        let mut data = HashMap::new();
        for i in 0..=max {
            data.insert(i, "0".repeat(i));
        }
        MoovIoAchStringZeros { data }
    }

    fn get(z: &MoovIoAchStringZeros, n: &usize) -> Option<String> {
        z.data.get(n).cloned()
    }
}

struct MoovIoAchAdvBatchControl {
    odfi_identification: String,
    // ... other fields ...
    moov_io_ach_converters: MoovIoAchConverters,
    string_zeros: Box<MoovIoAchStringZeros>,
}

impl MoovIoAchAdvBatchControl {
    fn odfi_identification_field(&self) -> String {
        self.odfi_identification.clone()
    }

    fn string_field(&self, s: &str, max: usize) -> String {
        self.moov_io_ach_converters.string_field(s, max, &self.string_zeros)
    }
}

impl fmt::Display for MoovIoAchAdvBatchControl {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "ODFIIdentification: {}",
            self.odfi_identification_field()
        )
    }
}

fn main() {
    let moov_io_ach_string_zeros = Box::new(MoovIoAchStringZeros::new(94));
    let moov_io_ach_converters = MoovIoAchConverters;

    let mut moov_io_ach_adv_batch_control = MoovIoAchAdvBatchControl {
        odfi_identification: "12345678".to_string(),
        // ... other fields ...
        moov_io_ach_converters,
        string_zeros: moov_io_ach_string_zeros,
    };

    println!("{}", moov_io_ach_adv_batch_control);
}

