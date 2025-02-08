

pub struct Test;

pub fn wskip(s: &str) -> String {
    s.chars()
        .skip_while(|c| c.is_whitespace())
        .collect()
}

