
use std::collections::HashMap;

fn jaccard_similarity(str1: &str, str2: &str, split_length: isize) -> f32 {
    if str1.is_empty() || str2.is_empty() {
        return 0.0;
    }

    let (splitted_str1, splitted_str2) = if split_length == 0 {
        (str1.split(' ').map(|s| s.to_string()).collect(), str2.split(' ').map(|s| s.to_string()).collect())
    } else {
        (shingle_slice(str1, split_length as usize), shingle_slice(str2, split_length as usize))
    };

    let runestr1: Vec<Vec<char>> = splitted_str1.iter().map(|s| s.chars().collect()).collect();
    let runestr2: Vec<Vec<char>> = splitted_str2.iter().map(|s| s.chars().collect()).collect();

    let union_str = union(&splitted_str1, &splitted_str2);
    let jacc = (runestr1.len() + runestr2.len() - union_str.len()) as f32;

    jacc / union_str.len() as f32
}

fn shingle_slice(s: &str, k: usize) -> Vec<String> {
    let mut out = Vec::new();
    let mut m = HashMap::new();
    if !s.is_empty() && k != 0 {
        let rune_s: Vec<char> = s.chars().collect();
        for i in 0..rune_s.len() - k + 1 {
            let shingle: String = rune_s[i..i + k].iter().collect();
            *m.entry(shingle).or_insert(0) += 1;
        }
        out = m.keys().cloned().collect();
    }
    out
}

fn union(a: &[String], b: &[String]) -> Vec<Vec<char>> {
    let mut m = HashMap::new();
    for item in a {
        m.insert(item.clone(), true);
    }
    for item in b {
        if !m.contains_key(item) {
            m.insert(item.clone(), true);
        }
    }

    let mut out: Vec<Vec<char>> = Vec::with_capacity(m.len());
    for word in m.keys() {
        out.push(word.chars().collect());
    }
    out
}
