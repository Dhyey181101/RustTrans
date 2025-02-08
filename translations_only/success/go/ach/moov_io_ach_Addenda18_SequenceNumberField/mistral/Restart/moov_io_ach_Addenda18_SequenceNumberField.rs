

use std::collections::HashMap;
use std::fmt;
use std::boxed::Box;

const ZEROS: &str = "0";

struct Addenda18 {
 sequence_number: i32, // added colon
 // ... other fields ...
 converters: Box<Converters>,
}

impl Addenda18 {
 fn sequence_number_field(&self) -> String {
 self.converters.numeric_field(self.sequence_number, 4)
 }
}

struct Converters {
 converter_map: HashMap<usize, String>,
}

impl Converters {
 fn numeric_field(&self, n: i32, max: u32) -> String {
 let s = n.to_string();
 let l = s.len() as u32;
 if l > max {
 return s[(l - max) as usize..].to_string();
 } else {
 let m = max - l;
 let pad = self.get_pad(m as usize);
 return format!("{}{}", pad, s);
 }
 }

 fn get_pad(&self, n: usize) -> String {
 let mut out = String::new();
 for _ in 0..n {
 out.push_str(ZEROS);
 }
 out
 }
}

// ... other code ...

