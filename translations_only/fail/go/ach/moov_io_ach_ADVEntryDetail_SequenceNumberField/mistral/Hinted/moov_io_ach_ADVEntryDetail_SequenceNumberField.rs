

use std::fmt;
use std::collections::HashMap;
use std::str;

const ZEROS: &[u8] = b"0000000000";

struct MoovIoAchAdvEntryDetail {
    sequence_number: i32,
}

struct MoovIoAchConverters;

impl MoovIoAchAdvEntryDetail {
    fn sequence_number_field(&self) -> String {
        MoovIoAchConverters::numeric_field(self.sequence_number, 4)
    }
}

impl MoovIoAchConverters {
    fn numeric_field(n: i32, max: u32) -> String {
        let s = n.to_string();
        let len = s.len() as u32;
        if len > max {
            s[(len-max) as usize..].to_string()
        } else {
            let m = max - len;
            let pad = get_zeros(m as usize);
            format!("{}{}", pad, s)
        }
    }
}

fn get_zeros(n: usize) -> String {
    if n > 0 {
        let zeros = &ZEROS[..n];
        String::from_utf8_lossy(zeros).to_string()
    } else {
        String::new()
    }
}

fn moov_io_ach_populate_map(max: i32, zero: char) -> HashMap<i32, String> {
    let mut out = HashMap::new();
    for i in 0..max {
        let z = (0..i).map(|_| zero).collect::<String>();
        out.insert(i, z);
    }
    out
}

impl fmt::Display for MoovIoAchAdvEntryDetail {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.sequence_number_field())
    }
}

