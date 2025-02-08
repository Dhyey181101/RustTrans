
use std::str;
use std::num::ParseIntError;
use std::usize;

fn read_uint(s: &str) -> Result<(i64, usize), ParseIntError> {
    let i = end(s);
    let x: i64 = match i64::from_str_radix(&s[..i], 10) {
        Ok(num) => num,
        Err(e) => return Err(e),
    };
    Ok((x, i))
}

fn end(s: &str) -> usize {
    s.chars().take_while(|c| !c.is_whitespace()).count()
}
