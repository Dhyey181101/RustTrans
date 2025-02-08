

use std::fmt;

fn HammingDistance(str1: &str, str2: &str) -> Result<isize, &'static str> {
// Convert strings to slice of u16 to handle no-ASCII characters
let rune_str1 = str1.chars().map(|c| c as u16).collect::<Vec<_>>();
let rune_str2 = str2.chars().map(|c| c as u16).collect::<Vec<_>>();

if rune_str1.len() != rune_str2.len() {
return Err("Undefined for strings of unequal length");
} else if Equal(&rune_str1, &rune_str2) {
return Ok(0);
}

let counter = rune_str1.iter()
.zip(rune_str2.iter())
.filter(|&(a, b)| a != b)
.count() as isize;

Ok(counter)
}

fn Equal(a: &[u16], b: &[u16]) -> bool {
if a.len() != b.len() {
return false;
}
for (i, v) in a.iter().enumerate() {
if v != &b[i] {
return false;
}
}
true
}

fn isize_to_string(i: &isize) -> String {
i.to_string()
}

