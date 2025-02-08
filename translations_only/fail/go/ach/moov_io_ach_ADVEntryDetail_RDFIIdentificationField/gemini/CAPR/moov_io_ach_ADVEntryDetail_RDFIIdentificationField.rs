
use std::collections::HashMap;
use std::fmt::Display;
use std::ops::Deref;

#[derive(Debug)]
struct MoovIoAchAdvEntryDetail {
    rdfi_identification: String,
}

impl MoovIoAchAdvEntryDetail {
    fn rdfi_identification_field(&self) -> &str {
        &self.rdfi_identification
    }
}

#[derive(Debug)]
struct MoovIoAchConverters {}

impl MoovIoAchConverters {
    fn string_field(&self, s: &str, max: u32) -> String {
        let ln = s.chars().count() as u32;
        if ln > max {
            return s[..max as usize].to_string();
        }

        let m = max - ln;
        let pad = moov_io_ach_string_zeros(m as usize);
        pad + s
    }
}

fn moov_io_ach_string_zeros(m: usize) -> String {
    let mut out = String::with_capacity(m);
    for _ in 0..m {
        out.push('0');
    }
    out
}

fn moov_io_ach_populate_map(max: usize, zero: &str) -> HashMap<usize, String> {
    let mut out = HashMap::with_capacity(max);
    for i in 0..max {
        out.insert(i, zero.repeat(i));
    }
    out
}

fn main() {
    let moov_io_ach_string_zeros = moov_io_ach_populate_map(94, "0");
    let moov_io_ach_converters = MoovIoAchConverters {};
    let ed = MoovIoAchAdvEntryDetail {
        rdfi_identification: "12345678".to_string(),
    };
    println!("{}", ed.rdfi_identification_field());
    println!("{}", moov_io_ach_converters.string_field("12345678", 8));
}
