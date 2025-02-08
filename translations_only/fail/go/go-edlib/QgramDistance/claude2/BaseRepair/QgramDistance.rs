
use std::collections::HashMap;

fn qgram_distance(str1: &str, str2: &str, split_length: isize) -> isize {
    let splitted_str1 = shingle(str1, split_length);
    let splitted_str2 = shingle(str2, split_length);

    let mut union = HashMap::new();
    for i in splitted_str1.keys() {
        union.insert(i.clone(), 0);
    }
    for i in splitted_str2.keys() {
        union.insert(i.clone(), 0);
    }

    let mut res = 0;

    for i in union.keys() {
        res += (splitted_str1.get(i).unwrap_or(&0) - splitted_str2.get(&*i).unwrap_or(&0)).abs() as isize;
    }

    res
}

fn shingle(s: &str, k: isize) -> HashMap<String, isize> {
    let mut m = HashMap::new();
    if !s.is_empty() && k != 0 {
        let rune_s: Vec<char> = s.chars().collect();

        for i in 0..(rune_s.len() as isize - k + 1) {
            *m.entry(rune_s[i as usize..(i + k) as usize].iter().collect()).or_insert(0) += 1;
        }
    }
    m
}

