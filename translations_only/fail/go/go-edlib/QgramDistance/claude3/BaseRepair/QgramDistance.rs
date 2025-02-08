
use std::collections::HashMap;

fn qgram_distance(str1: &str, str2: &str, split_length: usize) -> isize {
    let splitted_str1 = shingle(str1, split_length);
    let splitted_str2 = shingle(str2, split_length);

    let mut union = HashMap::new();
    for (k, v) in splitted_str1.iter() {
        union.insert(k.clone(), *v);
    }
    for (k, v) in splitted_str2.iter() {
        union.insert(k.clone(), *v);
    }

    let mut res = 0;

    for (k, _) in union.iter() {
        let val1 = *splitted_str1.get(k).unwrap_or(&0);
        let val2 = *splitted_str2.get(k).unwrap_or(&0);
        res += (val1 as isize - val2 as isize).abs();
    }

    res
}

fn shingle(s: &str, k: usize) -> HashMap<String, usize> {
    let mut m = HashMap::new();
    if !s.is_empty() && k != 0 {
        let rune_s: Vec<char> = s.chars().collect();

        for i in 0..rune_s.len() - k + 1 {
            let key: String = rune_s[i..i + k].iter().collect();
            *m.entry(key).or_insert(0) += 1;
        }
    }
    m
}
