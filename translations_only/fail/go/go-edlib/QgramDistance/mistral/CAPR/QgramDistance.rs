

use std::collections::HashMap;
use std::isize;

fn QgramDistance(str1: &str, str2: &str, split_length: isize) -> isize {
let splitted_str1 = Shingle(str1, split_length);
let splitted_str2 = Shingle(str2, split_length);

let mut union = HashMap::new();
for i in &splitted_str1 {
union.insert(i, 0);
}
for i in &splitted_str2 {
if union.contains_key(i) {
*union.get_mut(i).unwrap() += 1;
} else {
union.insert(i, 1);
}
}

let intersection: isize = union.values().filter(|v| **v > 0).count() as isize;
let union_size: isize = union.len() as isize;

isize::max(0, union_size - intersection)
}

fn Shingle(text: &str, gram_size: isize) -> Vec<&str> {
(0..text.len() - gram_size as usize + 1)
.map(|i| &text[i..(i + gram_size as usize)])
.collect()
}

