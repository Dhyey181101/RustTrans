
use std::collections::HashMap;
use std::fmt::Display;
use std::ops::Deref;
use std::str::FromStr;
use std::string::ToString;

#[derive(Debug, Clone)]
struct MoovIoAchAddenda99Contested {
    dishonored_return_reason_code: String,
}

impl MoovIoAchAddenda99Contested {
    fn dishonored_return_reason_code_field(&self) -> &str {
        &self.dishonored_return_reason_code
    }
}

#[derive(Debug, Clone)]
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

fn moov_io_ach_string_zeros(max: usize) -> String {
    let mut out = String::new();
    for i in 0..max {
        out.push_str(&"0".repeat(i));
    }
    out
}

fn main() {
    let addenda99_contested = MoovIoAchAddenda99Contested {
        dishonored_return_reason_code: "123456".to_string(),
    };

    let converters = MoovIoAchConverters {};

    let dishonored_return_reason_code_field = addenda99_contested.dishonored_return_reason_code_field();
    let string_field = converters.string_field(dishonored_return_reason_code_field, 2);

    println!("{}", string_field);
}
