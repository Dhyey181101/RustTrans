

use std::fmt;
use std::collections::HashMap;

struct MoovIoAchConverters;

impl MoovIoAchConverters {
 fn numeric_field(&self, n: i32, max: u32) -> String {
 let s = n.to_string();
 if s.len() as u32 > max {
 let start = s.len() as u32 - max;
 let end = s.len() as u32;
 let s_slice = &s[start as usize..end as usize];
 return s_slice.to_string();
 } else {
 let m = max as i32 - s.len() as i32;
 let pad = self.get_pad_string(m);
 return pad + &s;
 }
 }

 fn get_pad_string(&self, n: i32) -> String {
 let mut out = HashMap::new();
 for i in 0..=n {
 out.insert(i, "0".repeat(i as usize));
 }
 out[&n].clone()
 }
}

struct MoovIoAchAddenda1 {
 converters: Box<MoovIoAchConverters>,
}

impl MoovIoAchAddenda1 {
 fn new() -> MoovIoAchAddenda1 {
 MoovIoAchAddenda1 {
 converters: Box::new(MoovIoAchConverters),
 }
 }

 fn numeric_field(&self, n: i32, max: u32) -> String {
 self.converters.numeric_field(n, max)
 }

 fn get_pad_string(&self, n: i32) -> String {
 self.converters.get_pad_string(n)
 }
}

