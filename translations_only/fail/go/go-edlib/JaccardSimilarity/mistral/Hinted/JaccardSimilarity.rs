

use std::collections::HashMap;
use std::isize;

fn JaccardSimilarity(str1: &str, str2: &str, split_length: isize) -> f32 {
if str1.is_empty() || str2.is_empty() {
return 0.0;
}

let splitted_str1 = if split_length == 0 {
shingle(str1, 0)
} else {
shingle(str1, split_length as usize)
};

let splitted_str2 = if split_length == 0 {
shingle(str2, 0)
} else {
shingle(str2, split_length as usize)
};

let rune_str1: Vec<Vec<char>> = splitted_str1
.iter()
.map(|s| s.chars().collect())
.collect();
let rune_str2: Vec<Vec<char>> = splitted_str2
.iter()
.map(|s| s.chars().collect())
.collect();

let union_str = union(splitted_str1, splitted_str2);

let jacc = (rune_str1.len() + rune_str2.len() - union_str.len()) as f32;

jacc / union_str.len() as f32
}

fn shingle(s: &str, k: usize) -> Vec<String> {
let mut out = Vec::new();
let mut m = HashMap::new();

if s.is_empty() || k == 0 {
return out;
}

let rune_s: Vec<char> = s.chars().collect();

for i in 0..rune_s.len() - k + 1 {
let key = rune_s[i..i + k].iter().collect::<String>();
*m.entry(key).or_insert(0) += 1;
}

for (k, v) in m {
out.push(k);
}

out
}

fn union(a: Vec<String>, b: Vec<String>) -> Vec<Vec<char>> {
let mut m = HashMap::new();

for item in a {
m.insert(item, true);
}

for item in b {
if !m.contains_key(&item) {
m.insert(item, true);
}
}

let mut out = Vec::new();

for (_, v) in m {
out.push(v.to_string().chars().collect());
}

out
}

