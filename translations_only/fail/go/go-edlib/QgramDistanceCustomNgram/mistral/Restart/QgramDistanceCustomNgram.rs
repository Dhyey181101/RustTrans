

use std::collections::HashMap;
use std::i32;

fn QgramDistanceCustomNgram(
splitted_str1: HashMap<String, i32>,
splitted_str2: HashMap<String, i32>,
) -> i32 {
let mut union = splitted_str1.clone();
for (k, _) in &splitted_str2 {
union.insert(k.clone(), 0);
}

let mut res = 0;
for (_, v) in union {
let v1 = *splitted_str1.get(&v.to_string()).unwrap_or(&0);
let v2 = *splitted_str2.get(&v.to_string()).unwrap_or(&0);
res += i32::abs(v1 - v2);
}

res
}

