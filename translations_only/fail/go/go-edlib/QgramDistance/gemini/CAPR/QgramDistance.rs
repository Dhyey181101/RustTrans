
use std::collections::HashMap;

fn qgram_distance(str1: &str, str2: &str, split_length: usize) -> isize {
    let splitted_str1 = shingle(str1, split_length);
    let splitted_str2 = shingle(str2, split_length);

    let mut union: HashMap<String, isize> = HashMap::new();
    for i in splitted_str1.keys() {
        union.insert(i.to_string(), 0);
    }
    for i in splitted_str2.keys() {
        union.insert(i.to_string(), 0);
    }

    let mut res = 0;

    for i in union.keys() {
        res += (splitted_str1.get(i).unwrap() - splitted_str2.get(i).unwrap()).abs() as isize;
    }

    res
}

fn shingle(s: &str, k: usize) -> HashMap<String, isize> {
    let mut m: HashMap<String, isize> = HashMap::new();
    if s != "" && k != 0 {
        let rune_s: Vec<char> = s.chars().collect();

        for i in 0..rune_s.len() - k + 1 {
            m.insert(rune_s[i..i + k].iter().collect(), 1);
        }
    }
    m
}
