
use std::collections::HashMap;
use std::isize;

const NOT_PRESENT: isize = -1;

fn SorensenDiceCoefficient(str1: &str, str2: &str, split_length: isize) -> f32 {
if str1.is_empty() && str2.is_empty() {
return 0.0;
}

let mut map1 = HashMap::new();
let mut map2 = HashMap::new();

let mut i = 0;
while i + split_length as usize <= str1.len() {
let key = &str1[i..i + split_length as usize];
*map1.entry(key).or_insert(0) += 1;
i += split_length as usize;
}

let mut i = 0;
while i + split_length as usize <= str2.len() {
let key = &str2[i..i + split_length as usize];
*map2.entry(key).or_insert(0) += 1;
i += split_length as usize;
}

let mut intersection_count = 0;
let mut union_count = 0;

for (key, value) in &map1 {
match map2.get(key) {
Some(value2) => {
intersection_count += value.min(value2);
union_count += value.max(value2);
}
_ => {}
}
}

if union_count == 0 {
return 0.0;
}

(2.0 * intersection_count as f32 / union_count as f32) as f32
}
