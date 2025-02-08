

use std::collections::HashMap;
use std::fmt;
use std::str;

const MAX: usize = 94;

struct MoovIoAchConverters;

impl MoovIoAchConverters {
 fn numeric_field(&self, n: i32, max: u32) -> String {
 let s = n.to_string();
 let l = s.len() as u32;
 if l < max {
 let missing_zeros: Vec<u8> = vec![b'0'; (max - l) as usize];
 let missing_zeros_str = str::from_utf8(&missing_zeros).unwrap();
 let result = format!("{}{}", missing_zeros_str, s);
 result
 } else {
 s
 }
 }
}

fn main() {
 let moov_io_ach_converters = MoovIoAchConverters;
 let mut map: HashMap<String, String> = HashMap::new();
 map.insert("123".to_string(), moov_io_ach_converters.numeric_field(123, 5));
 map.insert("4567".to_string(), moov_io_ach_converters.numeric_field(4567, 4));
 for (k, v) in map.iter() {
 println!("Key: {}, Value: {}", k, v);
 }
}

