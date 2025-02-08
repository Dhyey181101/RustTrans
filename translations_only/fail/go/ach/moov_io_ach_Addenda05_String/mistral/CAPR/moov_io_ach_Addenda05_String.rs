

use std::fmt;
use std::iter;

const RECORD_LENGTH: usize = 94;
const ENTRY_ADDENDA_POS: &str = "7";

fn moov_io_ach_populate_map(max: usize, zero: char) -> std::collections::HashMap<usize, String> {
    let mut out: std::collections::HashMap<usize, String> = std::collections::HashMap::new();
    for i in 0..max {
        out.insert(i, iter::repeat(zero).take(i).collect::<String>());
    }
    out
}

struct MoovIoAchConverters;

impl MoovIoAchConverters {
    fn alpha_field(&self, s: &str, max: usize) -> String {
        let ln: usize = s.chars().count();
        if ln > max {
            s[..max].to_string()
        } else {
            let m = max - ln;
            let pad = MoovIoAchAddenda0::get_pad_string(&MoovIoAchAddenda0::default(), m);
            s.to_string() + &pad
        }
    }

    fn numeric_field(&self, n: i64, max: usize) -> String {
        let s = n.to_string();
        let l = s.len();
        if l > max as usize {
            s[l - max as usize..].to_string()
        } else {
            let m = max - l;
            let pad = MoovIoAchAddenda0::get_pad_string(&MoovIoAchAddenda0::default(), m as usize);
            pad + &s
        }
    }
}

#[derive(Default)]
struct MoovIoAchAddenda0 {
    pad_string: String,
}

impl MoovIoAchAddenda0 {
    fn get_pad_string(&self, len: usize) -> String {
        self.pad_string[..len].to_string()
    }

    fn new(len: usize) -> Self {
        MoovIoAchAddenda0 {
            pad_string: " ".repeat(len),
        }
    }
}

