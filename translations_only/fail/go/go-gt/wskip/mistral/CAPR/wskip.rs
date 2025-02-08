

use std::boxed::Box;

pub struct Constants;

pub fn wskip(s: &str) -> String {
    let c: Box<String> = Box::new(
        s.chars()
            .skip_while(|c| c.is_whitespace())
            .collect::<String>(),
    );
    c.trim().to_string()
}

