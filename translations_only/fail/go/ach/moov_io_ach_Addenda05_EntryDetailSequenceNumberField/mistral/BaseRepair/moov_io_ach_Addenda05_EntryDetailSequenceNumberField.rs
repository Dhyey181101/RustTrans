

use std::collections::HashMap;
use std::fmt;
use std::str;

const ZEROS: &str = "0";

struct MoovIoAchConverters;

impl MoovIoAchConverters {
    fn numeric_field(&self, n: i32, max: u32) -> String {
        let s = n.to_string();
        let l = s.len() as u32;
        if l > max {
            return s[(l - max) as usize..].to_string();
        } else {
            let m = max - l;
            let pad = MoovIoAchAddenda05::get_pad(m as usize);
            return pad + &s;
        }
    }
}

struct MoovIoAchAddenda05 {
    entry_detail_sequence_number: i32,
    moov_io_ach_converters: MoovIoAchConverters,
}

impl MoovIoAchAddenda05 {
    fn get_pad(num: usize) -> String {
        let zeros = ZEROS.repeat(num);
        zeros
    }
}

