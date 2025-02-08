
use std::collections::HashMap;
use std::fmt;

const ZERO: char = '0';

struct MoovIoAchConverters;

impl MoovIoAchConverters {
    fn string_field(&self, s: &str, max: usize) -> String {
        let ln = s.len();
        if ln > max {
            return s[..max].to_string();
        }

        let m = max - ln;
        let pad = Self::get_pad_string(m);
        pad.chars().chain(s.chars()).collect()
    }

    fn get_pad_string(n: usize) -> String {
        let mut map = HashMap::new();
        for i in 0..n {
            map.insert(i, "0".repeat(i));
        }
        map.get(&n).unwrap().to_string()
    }
}

struct MoovIoAchIATEntryDetail {
    rdfi_identification: String,
}

impl MoovIoAchIATEntryDetail {
    fn rdfi_identification_field(&self) -> String {
        self.string_field(&self.rdfi_identification, 8)
    }

    fn string_field(&self, s: &str, max: usize) -> String {
        MoovIoAchConverters.string_field(s, max)
    }
}

impl fmt::Display for MoovIoAchIATEntryDetail {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "RDFIIdentification: {}",
            self.rdfi_identification_field()
        )
    }
}

fn main() {
    let iat_entry = MoovIoAchIATEntryDetail {
        rdfi_identification: "123456789".to_string(),
    };
    println!("{}", iat_entry);
}
