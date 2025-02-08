

use std::collections::HashMap;
use std::str;

fn JaccardSimilarity(str1: &str, str2: &str, split_length: usize) -> f32 {
if str1.is_empty() || str2.is_empty() {
return 0.0;
}

let splitted_str1 = if split_length == 0 {
shingle(str1, split_length)
} else {
ShingleSlice(str1, split_length)
};
let splitted_str2 = if split_length == 0 {
shingle(str2, split_length)
} else {
ShingleSlice(str2, split_length)
};

let rune_str1 = splitted_str1
.iter()
.map(|s| s.chars().collect::<Vec<char>>())
.collect::<Vec<Vec<char>>>();
let rune_str2 = splitted_str2
.iter()
.map(|s| s.chars().collect::<Vec<char>>())
.collect::<Vec<Vec<char>>>();

let union_str = union(&splitted_str1, &splitted_str2);

let jacc = (rune_str1.len() + rune_str2.len() - union_str.len()) as f32;

jacc / union_str.len() as f32
}

fn ShingleSlice(s: &str, k: usize) -> Vec<String> {
let mut out = Vec::new();
let mut m = HashMap::new();
if !s.is_empty() && k != 0 {
let rune_s: Vec<char> = s.chars().collect();
for i in 0..rune_s.len() - k + 1 {
let key = rune_s[i..i + k].iter().collect::<String>();
m.insert(key, 1);
}
for (k, _) in m {
out.push(k);
}
}
out
}

fn union(a: &Vec<String>, b: &Vec<String>) -> Vec<Vec<char>> {
let mut m = HashMap::new();
for item in a {
m.insert(item.clone(), true);
}
for item in b {
if !m.contains_key(item) {
// a.push(item.to_string());
m.insert(item.clone(), true);
}
}

let out: Vec<Vec<char>> = m.keys()
.map(|word| word.chars().collect::<Vec<char>>())
.collect();
out
}

fn shingle(s: &str, k: usize) -> Vec<String> {
let mut out = Vec::new();
let mut m = HashMap::new();
if !s.is_empty() && k != 0 {
let rune_s: Vec<char> = s.chars().collect();
for i in 0..rune_s.len() - k + 1 {
let key = rune_s[i..i + k].iter().collect::<String>();
m.insert(key, 1);
}
for (k, _) in m {
out.push(k);
}
}
out
}

