
use std::fmt;

fn HammingDistance(str1: &str, str2: &str) -> Result<isize, &'static str> {
// Convert strings to slice of u8
let slice1 = str1.as_bytes();
let slice2 = str2.as_bytes();

if slice1.len() != slice2.len() {
return Err("str1 and str2 are not of the same length");
}

let mut distance: isize = 0;
for (c1, c2) in slice1.iter().zip(slice2) {
if c1 != c2 {
distance += 1;
}
}

Ok(distance)
}
