

use std::fmt;
use std::collections::HashMap;

struct MoovIoAchConverters;

impl MoovIoAchConverters {
 fn numeric_field(&self, n: i32, max: u32) -> String {
 let s = n.to_string();
 if s.len() as u32 > max {
 format!("ERROR: number {} is too big for max size {}", n, max)
 } else {
 s
 }
 }
}

impl fmt::Display for MoovIoAchConverters {
 fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
 write!(f, "MoovIoAchConverters")
 }
}

