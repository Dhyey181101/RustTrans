
use std::collections::HashMap;

fn jaccard_similarity(str1: &str, str2: &str, split_length: isize) -> f32 {
    if str1.is_empty() || str2.is_empty() {
        return 0.0;
    }

    let mut splitted_str1 = Vec::new();
    let mut splitted_str2 = Vec::new();
    if split_length == 0 {
        splitted_str1 = str1.split_whitespace().map(|s| s.to_string()).collect();
        splitted_str2 = str2.split_whitespace().map(|s| s.to_string()).collect();
    } else {
        splitted_str1 = shingle_slice(str1, split_length as usize);
        splitted_str2 = shingle_slice(str2, split_length as usize);
    }

    let rune_str1 = splitted_str1.iter().map(|s| s.chars().collect::<Vec<_>>()).collect::<Vec<_>>();
    let rune_str2 = splitted_str2.iter().map(|s| s.chars().collect::<Vec<_>>()).collect::<Vec<_>>();

    let union_str = union(&splitted_str1, &splitted_str2);
    let jacc = (rune_str1.len() + rune_str2.len() - union_str.len()) as f32 / union_str.len() as f32;

    jacc
}

fn shingle_slice(s: &str, k: usize) -> Vec<String> {
    let mut out = Vec::new();
    let mut m = HashMap::new();
    if !s.is_empty() && k != 0 {
        let rune_s = s.chars().collect::<Vec<_>>();
        for i in 0..rune_s.len() - k + 1 {
            *m.entry(rune_s[i..i+k].iter().collect::<String>()).or_insert(0) += 1; 
        }
        for (k, _) in m {
            out.push(k);
        }
    }
    out
}

fn union(a: &[String], b: &[String]) -> Vec<Vec<char>> {
    let mut m = HashMap::new();
    for item in a {
        m.insert(item.clone(), true);
    }
    let mut union = a.to_vec();
    for item in b {
        if !m.contains_key(item) {
             union.push(item.clone());
        }
    }

    union.iter().map(|word| word.chars().collect::<Vec<_>>()).collect()
}
