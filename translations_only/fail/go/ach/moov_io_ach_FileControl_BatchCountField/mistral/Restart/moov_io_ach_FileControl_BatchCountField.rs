

use std::fmt;
use std::collections::HashMap;
use std::str;
use std::iter;

struct MoovIoAchConverters;

impl MoovIoAchConverters {
 fn numeric_field(&self, n: i32, max: u32) -> String {
 let s = n.to_string();
 if s.len() as u32 > max {
 s[s.len() - max as usize..].to_string()
 } else {
 let m = (max - s.len() as u32) as usize;
 iter::repeat('0').take(m).chain(s.chars()).collect()
 }
 }
}

impl fmt::Display for MoovIoAchConverters {
 fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
 write!(f, "MoovIoAchConverters")
 }
}

