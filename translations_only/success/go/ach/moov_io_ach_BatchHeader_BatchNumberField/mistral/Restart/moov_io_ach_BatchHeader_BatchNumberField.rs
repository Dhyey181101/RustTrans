

use std::collections::HashMap;
use std::fmt;
use std::str;

const ZEROS: &[u8] = b"000000000";

struct MoovIoAchBatchHeader {
    batch_number: i32,
    converters: Box<MoovIoAchConverters>,
}

struct MoovIoAchConverters;

impl MoovIoAchBatchHeader {
    fn batch_number_field(&self) -> String {
        self.converters.numeric_field(self.batch_number, 7)
    }
}

impl MoovIoAchConverters {
    fn numeric_field(&self, n: i32, max: u32) -> String {
        let s = n.to_string();
        let len = s.len() as u32;
        if len > max {
            s[(len - max) as usize..].to_string()
        } else {
            let padding_len = max - len;
            let padding = get_padding(padding_len as usize);
            format!("{}{}", padding, s)
        }
    }
}

fn get_padding(len: usize) -> String {
    if len > ZEROS.len() {
        "".to_string()
    } else {
        str::from_utf8(&ZEROS[..len]).unwrap().to_string()
    }
}

fn main() {}

