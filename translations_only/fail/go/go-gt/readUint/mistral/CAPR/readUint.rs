

use std::str;

fn read_uint(s: &str) -> (i64, usize) {
let i = end(s);
let x: i64 = match s[..i].parse() {
Ok(num) => num,
Err(_) => panic!("Invalid integer"),
};
return (x, i);
}

fn end(s: &str) -> usize {
s.chars().take_while(|c| !c.is_whitespace()).count()
}

