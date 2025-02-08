
use std::collections::HashMap;
use std::fmt::Display;
use std::ops::Deref;
use std::str::FromStr;

#[derive(Debug)]
struct MoovIoAchAddenda99Contested {
    return_reason_code: String,
}

impl MoovIoAchAddenda99Contested {
    fn return_reason_code_field(&self) -> &str {
        &self.return_reason_code
    }
}

struct MoovIoAchConverters {}

impl MoovIoAchConverters {
    fn string_field(&self, s: &str, max: usize) -> String {
        let ln = s.chars().count();
        if ln > max {
            return s[..max].to_string();
        }

        let m = max - ln;
        let pad = moov_io_ach_string_zeros(m);
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
    let mut addenda99_contested = MoovIoAchAddenda99Contested {
        return_reason_code: "123456".to_string(),
    };

    let converters = MoovIoAchConverters {};

    let return_reason_code_field = addenda99_contested.return_reason_code_field();
    let string_field = converters.string_field(return_reason_code_field, 2);

    println!("{}", string_field);
}
