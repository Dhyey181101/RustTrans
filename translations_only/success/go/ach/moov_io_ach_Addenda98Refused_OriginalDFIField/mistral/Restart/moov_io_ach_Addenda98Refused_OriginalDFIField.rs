

use std::collections::HashMap;
use std::fmt;

const ZEROS: &[u8] = b"0000000000000000000000000000000000000000000000000000000000000000";

struct Addenda98Refused {
    original_dfi: String,
    // ... other fields ...
    converters: Box<Converters>,
}

impl Addenda98Refused {
    fn original_dfi_field(&self) -> String {
        self.converters.string_field(&self.original_dfi, 8)
    }
}

struct Converters {
    _private_field: (),
}

impl Converters {
    fn string_field(&self, s: &str, max: usize) -> String {
        let ln = s.len();
        if ln > max {
            s[..max].to_string()
        } else {
            let m = max - ln;
            let pad = get_zeros(m);
            String::from_utf8_lossy(pad).to_string() + s
        }
    }
}

fn get_zeros(n: usize) -> &'static [u8] {
    if n < ZEROS.len() {
        &ZEROS[..n]
    } else {
        ZEROS
    }
}

fn main() {
    // ...
}

