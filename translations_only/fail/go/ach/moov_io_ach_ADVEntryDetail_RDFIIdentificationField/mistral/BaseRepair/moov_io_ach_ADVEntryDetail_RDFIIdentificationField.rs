

use std::collections::HashMap;
use std::fmt;

const ZEROS: &[u8] = b"0000000000000000000000000000000000000000000000000000000000000000";

struct MoovIoAchAdvEntryDetail {
    rdfi_identification: String,
}

impl MoovIoAchAdvEntryDetail {
    fn rdfi_identification_field(&self) -> String {
        let moov_io_ach_converters = MoovIoAchConverters;
        moov_io_ach_converters.string_field(&self.rdfi_identification, 8)
    }
}

struct MoovIoAchConverters;

impl MoovIoAchConverters {
    fn string_field(&self, s: &str, max: u32) -> String {
        let len = s.len() as u32;
        if len > max {
            return s[..max as usize].to_string();
        }

        let m = (max - len) as usize;
        let pad = get_zeros(m);
        String::from_utf8_lossy(pad).to_string() + s
    }
}

fn get_zeros(n: usize) -> &'static [u8] {
    if n < 8 {
        &ZEROS[..n]
    } else {
        &ZEROS[..]
    }
}

fn populate_map(max: i32, zero: char) -> HashMap<i32, String> {
    let mut out = HashMap::new();
    for i in 0..max {
        let zeros = (0..i).map(|_| zero).collect::<String>();
        out.insert(i, zeros);
    }
    out
}

impl fmt::Display for MoovIoAchAdvEntryDetail {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "MoovIoAchAdvEntryDetail(\
             rdfi_identification = {:?})\
            ",
            self.rdfi_identification
        )
    }
}

fn main() {
    let mut map = populate_map(94, '0');
    let entry = MoovIoAchAdvEntryDetail {
        rdfi_identification: String::from("12345678"),
    };

    println!("{}", entry.rdfi_identification_field());
}

