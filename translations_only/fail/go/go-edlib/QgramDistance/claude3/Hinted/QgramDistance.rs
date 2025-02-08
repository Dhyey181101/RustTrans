
use std::collections::HashMap;

fn qgram_distance(str1: &str, str2: &str, split_length: isize) -> isize {
    let splitted_str1 = shingle(str1, split_length as usize);
    let splitted_str2 = shingle(str2, split_length as usize);

    let mut union = HashMap::new();
    for (k, v) in splitted_str1.iter() {
        union.insert(k.to_string(), 0);
    }
    for (k, v) in splitted_str2.iter() {
        union.insert(k.to_string(), 0);
    }

    let mut res = 0;

    for k in union.keys() {
        res += (splitted_str1.get(k).unwrap_or(&0) - splitted_str2.get(k).unwrap_or(&0)).abs();
    }

    res
}

fn shingle(s: &str, k: usize) -> HashMap<String, isize> {
    let mut m = HashMap::new();
    if !s.is_empty() && k != 0 {
        let mut chars = s.chars();

        let mut window = Vec::with_capacity(k);
        for _ in 0..k {
            if let Some(c) = chars.next() {
                window.push(c);
            } else {
                break;
            }
        }

        while !window.is_empty() {
            let key: String = window.iter().collect();
            *m.entry(key).or_insert(0) += 1;

            if let Some(c) = chars.next() {
                window.remove(0);
                window.push(c);
            } else {
                break;
            }
        }
    }
    m
}
