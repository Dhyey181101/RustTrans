

use std::collections::HashMap;
use std::fmt;
use std::str;

const ZEROS: &[u8] = b"0000000000000000000000000000000000000000000000000000000000000000";

struct MoovIoAchConverters;

impl MoovIoAchConverters {
    fn numeric_field(&self, n: i32, max: u32) -> String {
        let s = n.to_string();
        if s.len() as u32 > max {
            return s[(s.len() as u32) as usize - (max as usize)..].to_string();
        }
        let m = max as usize - s.len();
        let pad = get_zeros(m);
        String::from_utf8_lossy(pad).to_string() + &s
    }
}

fn get_zeros(n: usize) -> &'static [u8] {
    if n > ZEROS.len() {
        return ZEROS;
    }
    &ZEROS[..n]
}

