

use std::boxed::Box;

pub struct Constants;

pub fn wskip(s: &str) -> String {
    let c = s.chars()
        .skip_while(|c| c.is_whitespace())
        .collect::<Vec<char>>();
    String::from_iter(c.into_iter()).trim().to_string()
}

