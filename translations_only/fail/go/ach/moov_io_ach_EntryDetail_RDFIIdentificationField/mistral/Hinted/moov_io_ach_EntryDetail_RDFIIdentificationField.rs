

use std::collections::HashMap;
use std::fmt;

const ZERO: char = '0';

struct MoovIoAchEntryDetail {
    rdfi_identification: String,
    // ... other fields omitted for brevity
}

struct MoovIoAchConverters;

impl MoovIoAchConverters {
    fn string_field(&self, s: &str, max: u32) -> String {
        let ln = s.len() as u32;
        if ln > max {
            return s[0..max as usize].to_string();
        }

        let m = (max - ln) as usize;
        let pad = self.get_zeros(m);
        pad + s
    }

    fn get_zeros(&self, count: usize) -> String {
        let mut zeros = String::from_iter(std::iter::repeat(ZERO).take(count));
        zeros.shrink_to_fit();
        zeros
    }
}

fn moov_io_ach_populate_map(max: usize, zero: char) -> HashMap<usize, String> {
    let mut out = HashMap::new();
    for i in 0..max {
        let zeros = String::from_iter(std::iter::repeat(zero).take(i));
        out.insert(i, zeros);
    }
    out
}

impl fmt::Display for MoovIoAchEntryDetail {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let converters = MoovIoAchConverters {};
        write!(
            f,
            "RDFIIdentification: {}",
            converters.string_field(&self.rdfi_identification, 8)
        )
    }
}

struct MoovIoAchConvertersWrapper;

impl MoovIoAchConvertersWrapper {
    fn new() -> Box<MoovIoAchConverters> {
        Box::new(MoovIoAchConverters {})
    }
}

fn main() {
    let mut map = moov_io_ach_populate_map(94, ZERO);
    let converters = MoovIoAchConvertersWrapper::new();
    let entry_detail = MoovIoAchEntryDetail {
        rdfi_identification: "123456789".to_string(),
        // ... other fields omitted for brevity
    };

    println!("{}", entry_detail);
}

