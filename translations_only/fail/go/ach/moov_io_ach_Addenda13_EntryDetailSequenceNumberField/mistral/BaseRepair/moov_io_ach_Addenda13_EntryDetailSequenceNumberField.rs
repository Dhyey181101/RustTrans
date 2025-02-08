

use std::collections::HashMap;
use std::fmt;
use std::str;

const ZEROS: &str = "0";

struct MoovIoAchConverters;

impl MoovIoAchConverters {
 fn numeric_field(&self, n: i32, max: u32) -> String {
 let s = n.to_string();
 let l = s.len() as u32;
 if l > max {
 s[l as usize - (max as usize)..].to_string()
 } else {
 let m = max - l;
 let pad = MoovIoAchAddenda1::get_zeros(m as usize);
 pad + &s
 }
 }
}

struct MoovIoAchAddenda1;

impl MoovIoAchAddenda1 {
 fn get_zeros(n: usize) -> String {
 let zeros = ZEROS.repeat(n);
 zeros
 }
}

