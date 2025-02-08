

use std::collections::HashMap;
use std::fmt;
use std::str;

const ZEROS: &str = "0";

struct MoovIoAchConverters;

impl MoovIoAchConverters {
fn numeric_field(&self, n: i32, max: u32) -> String {
let s = n.to_string();
let l = s.len() as u32;
let padding = "0123456789"
.repeat(max as usize)
.chars()
.skip((l - 1) as usize)
.collect::<String>();
return format!("{}{}", padding, s);
}
}

impl fmt::Display for MoovIoAchConverters {
fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
write!(f, "MoovIoAchConverters")
}
}

