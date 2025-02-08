

use std::collections::HashMap;
use std::fmt;

struct MoovIoAchConverters {
    moov_io_ach_string_zeros: HashMap<usize, String>,
}

impl MoovIoAchConverters {
    fn new() -> Self {
        MoovIoAchConverters {
            moov_io_ach_string_zeros: <HashMap<usize, String>>::default(),
        }
    }

    fn string_field(&self, s: &str, max: usize) -> String {
        let ln = s.len();
        if ln > max {
            s[..max].to_string()
        } else {
            let m = max - ln;
            let pad = self
                .moov_io_ach_string_zeros
                .get(&m)
                .cloned()
                .unwrap_or_else(|| "0".repeat(m));
            pad + s
        }
    }

    fn parse_string_field(&self, r: &str) -> String {
        r.trim().to_string()
    }
}

impl Default for MoovIoAchConverters {
    fn default() -> Self {
        MoovIoAchConverters::new()
    }
}

struct MoovIoAchAdvEntryDetail {
    rdfi_identification: String,
    check_digit: String,
    // ... other fields
    moov_io_ach_converters: MoovIoAchConverters,
}

impl MoovIoAchAdvEntryDetail {
    fn set_rdfi(&mut self, rdfi: &str) -> &mut Self {
        let s = self.moov_io_ach_converters.string_field(rdfi, 9);
        self.rdfi_identification = self
            .moov_io_ach_converters
            .parse_string_field(&s[..8]);
        self.check_digit = self
            .moov_io_ach_converters
            .parse_string_field(&s[8..9]);
        self
    }
}

impl fmt::Display for MoovIoAchAdvEntryDetail {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "RDFIIdentification: {}, CheckDigit: {}",
            self.rdfi_identification, self.check_digit
        )
    }
}

fn main() {
    let mut ed = MoovIoAchAdvEntryDetail {
        rdfi_identification: String::new(),
        check_digit: String::new(),
        moov_io_ach_converters: Default::default(),
    };
    let rdfi = "123456789";
    ed.set_rdfi(rdfi);
    println!("{}", ed);
}

fn moov_io_ach_populate_map(max: usize, zero: char) -> HashMap<usize, String> {
    let mut out = HashMap::new();
    for i in 0..max {
        out.insert(i, "0".repeat(i));
    }
    out
}

struct MyHashMap {
    data: HashMap<usize, String>,
}

impl Default for MyHashMap {
    fn default() -> Self {
        MyHashMap {
            data: moov_io_ach_populate_map(94, '0'),
        }
    }
}

