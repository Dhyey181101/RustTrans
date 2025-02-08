

use std::fmt;
use std::collections::HashMap;

struct MoovIoAchConverters;

impl MoovIoAchConverters {
 fn numeric_field(&self, n: i32, max: u32) -> String {
 let s = n.to_string();
 if s.len() as u32 > max {
 let start_index = (s.len() as u32) - max;
 let sliced_s: &str = &s[start_index as usize..];
 return sliced_s.to_string();
 } else {
 let m = max as i32 - s.len() as i32;
 format!("{}{}", "0".repeat(m as usize), s)
 }
 }
}

impl fmt::Display for MoovIoAchConverters {
 fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
 unimplemented!()
 }
}

