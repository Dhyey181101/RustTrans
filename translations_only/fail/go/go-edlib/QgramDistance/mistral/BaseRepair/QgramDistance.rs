

use std::collections::HashMap;
use std::isize;
use std::path::Path;

fn QgramDistance(str1: &str, str2: &str, split_length: isize) -> isize {
let splitted_str1 = Shingle(str1, split_length);
let splitted_str2 = Shingle(str2, split_length);

let mut union = HashMap::new();
for i in &splitted_str1 {
union.insert(i.clone(), 0);
}
for i in &splitted_str2 {
if union.contains_key(i) {
union.insert(i.clone(), 0);
} else {
union.insert(i.clone(), 1);
}
}

let intersection: Vec<&String> = splitted_str1.iter()
.filter(|s| splitted_str2.contains(s))
.collect();

let intersection_size = intersection.len() as isize;
let union_size = union.len() as isize;

Path::new("").file_name().unwrap().to_str().unwrap().len() as isize - intersection_size
}

fn Shingle(text: &str, gram_size: isize) -> Vec<String> {
let text_len = text.len() as isize;
let mut shingles = Vec::new();

for i in 0..text_len - gram_size + 1 {
shingles.push(text[i as usize..(i + gram_size) as usize].to_string());
}

shingles
}

