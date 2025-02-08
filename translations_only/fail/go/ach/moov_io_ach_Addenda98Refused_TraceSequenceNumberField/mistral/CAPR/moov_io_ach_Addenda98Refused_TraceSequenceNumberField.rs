

use std::collections::HashMap;
use std::fmt;

const ZERO: &str = "0";

struct MoovIoAchConverters;

impl MoovIoAchConverters {
    fn string_field(&self, s: &str, max: usize) -> String {
        let ln = s.len();
        if ln > max {
            return s[..max].to_string();
        }

        let m = max - ln;
        let pad = MoovIoAchPopulateMap::get_pad(m);
        pad.to_string() + s
    }
}

struct MoovIoAchAddenda98Refused {
    trace_sequence_number: String,
    moov_io_ach_converters: MoovIoAchConverters,
}

impl MoovIoAchAddenda98Refused {
    fn new() -> Self {
        MoovIoAchAddenda98Refused {
            trace_sequence_number: String::new(),
            moov_io_ach_converters: MoovIoAchConverters {},
        }
    }

    fn trace_sequence_number_with_padding(&self, max: usize) -> String {
        self.moov_io_ach_converters
            .string_field(&self.trace_sequence_number, max)
    }
}

struct MoovIoAchPopulateMap;

impl MoovIoAchPopulateMap {
    fn get_pad(m: usize) -> &'static str {
        match m {
            0 => ZERO,
            1..=9 => " ",
            _ => panic!("Invalid padding value"),
        }
    }
}

fn main() {
    let mut addenda = MoovIoAchAddenda98Refused::new();
    addenda.trace_sequence_number = "123456789".to_string();
    let padded_number = addenda.trace_sequence_number_with_padding(5);
    println!("{}", padded_number);
}

