

use std::fmt;
use std::collections::HashMap;
use std::boxed::Box;

const ZEROS: &[u8] = b"0000000000";

struct MoovIoAchAdvEntryDetail {
    sequence_number: i32,
}

struct MoovIoAchConverters;

impl MoovIoAchAdvEntryDetail {
    fn sequence_number_field(&self) -> String {
        MoovIoAchConverters::numeric_field(&self.sequence_number, 4)
    }
}

impl MoovIoAchConverters {
    fn numeric_field(n: &i32, max: u32) -> String {
        let s = n.to_string();
        let len = s.len() as u32;
        if len > max {
            s[(len-max) as usize..].to_string()
        } else {
            let m = max - len;
            let pad = get_zeros(m as usize);
            let mut v = Vec::new();
            v.extend_from_slice(pad);
            v.extend_from_slice(s.as_bytes());
            String::from_utf8(v).unwrap()
        }
    }
}

fn get_zeros(n: usize) -> &'static [u8] {
    if n > 0 && n < 10 {
        unsafe { &ZEROS[..n] }
    } else {
        b"0000000000"
    }
}

impl fmt::Display for MoovIoAchAdvEntryDetail {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.sequence_number_field())
    }
}

fn main() {
    let adv = Box::new(MoovIoAchAdvEntryDetail { sequence_number: 12345 });
    println!("{}", adv);
}

