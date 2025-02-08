
use std::fmt;
use std::iter;

const RECORD_LENGTH: usize = 94;
const ENTRY_ADDENDA_POS: &str = "7";

struct MoovIoAchConverters;

impl MoovIoAchConverters {
    fn alpha_field(&self, s: &str, max: usize) -> String {
        let ln = s.len();
        if ln > max {
            s[..max].to_string()
        } else {
            let m = max - ln;
            let pad = Self::get_pad_string(m, ' ');
            format!("{}{}", s, pad)
        }
    }

    fn numeric_field(&self, n: i32, max: usize) -> String {
        let s = n.to_string();
        let l = s.len();
        if l > max {
            s[l - max..].to_string()
        } else {
            let m = max - l;
            let pad = Self::get_pad_string(m, '0');
            format!("{}{}", pad, s)
        }
    }

    fn get_pad_string(n: usize, c: char) -> String {
        iter::repeat(c).take(n).collect::<String>()
    }
}

#[derive(Default)]
struct MoovIoAchAddenda1 {
    data: Option<String>,
}
