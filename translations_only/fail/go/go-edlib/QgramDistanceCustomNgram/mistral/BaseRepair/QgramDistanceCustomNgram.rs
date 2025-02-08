

use std::collections::HashMap;
use std::i32;

fn QgramDistanceCustomNgram(
splitted_str1: HashMap<String, i32>,
splitted_str2: HashMap<String, i32>,
) -> i32 {
let mut union = splitted_str1.clone();
union.extend(splitted_str2.clone());

let mut res = 0;
for i in &union {
res += i32::abs((*splitted_str1.get(i.0).unwrap_or(&0)) as i32 - (*splitted_str2.get(i.0).unwrap_or(&0))) as i32;
}

res
}

