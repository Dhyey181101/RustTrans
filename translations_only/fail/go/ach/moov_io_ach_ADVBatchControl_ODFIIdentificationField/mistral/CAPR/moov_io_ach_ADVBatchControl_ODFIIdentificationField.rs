

use std::collections::HashMap;
use std::fmt;

const ZERO: char = '0';

struct MoovIoAchADVBatchControl {
    odfi_identification: String,
    // ... other fields ...
    moov_io_ach_converters: MoovIoAchConverters,
}

struct MoovIoAchConverters;

impl MoovIoAchADVBatchControl {
    fn odfi_identification_field(&self) -> String {
        self.odfi_identification.clone()
    }
}

impl MoovIoAchConverters {
    fn string_field(&self, s: &str, max: usize) -> String {
        let len = s.chars().count();
        if len > max {
            s[..max].to_string()
        } else {
            let zeros = get_zeros(max - len);
            zeros + s
        }
    }
}

fn get_zeros(n: usize) -> String {
    let mut zeros = String::with_capacity(n);
    for _ in 0..n {
        zeros.push(ZERO);
    }
    zeros
}

fn populate_map(max: usize, zero: char) -> HashMap<usize, String> {
    let mut out = HashMap::new();
    for i in 0..max {
        let zeros = get_zeros(i);
        out.insert(i, zeros);
    }
    out
}

// Implement Display trait for MoovIoAchADVBatchControl
impl fmt::Display for MoovIoAchADVBatchControl {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "ODFIIdentification: {}",
            self.odfi_identification_field()
        )
    }
}

fn main() {
    let mut map = populate_map(94, ZERO);
    let bc = MoovIoAchADVBatchControl {
        odfi_identification: "12345678".to_string(),
        moov_io_ach_converters: MoovIoAchConverters {},
    };
    println!("{}", bc);
}

