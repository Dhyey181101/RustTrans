
use std::collections::HashMap;

fn jaccard_similarity(str1: &str, str2: &str, split_length: usize) -> f32 {
    if str1.is_empty() || str2.is_empty() {
        return 0.0;
    }

    let (splitted_str1, splitted_str2) = if split_length == 0 {
        (str1.split_whitespace().map(|s| s.to_string()).collect(), str2.split_whitespace().map(|s| s.to_string()).collect())
    } else {
        (shingle_slice(str1, split_length), shingle_slice(str2, split_length))
    };

    let rune_str1: Vec<Box<[char]>> = splitted_str1.iter().map(|s| s.chars().collect::<Vec<_>>().into_boxed_slice()).collect();
    let rune_str2: Vec<Box<[char]>> = splitted_str2.iter().map(|s| s.chars().collect::<Vec<_>>().into_boxed_slice()).collect();

    let union_str = union(&splitted_str1, &splitted_str2);
    let jacc = (rune_str1.len() + rune_str2.len() - union_str.len()) as f32;

    jacc / union_str.len() as f32
}

fn shingle_slice(s: &str, k: usize) -> Vec<String> {
    let mut out = Vec::new();
    let mut m = HashMap::new();
    if !s.is_empty() && k != 0 {
        let chars: Vec<char> = s.chars().collect();
        for i in 0..chars.len() - k + 1 {
            let shingle: String = chars[i..i + k].iter().collect();
            *m.entry(shingle).or_insert(0) += 1;
        }
        out.extend(m.keys().cloned());
    }
    out
}

fn union(a: &[String], b: &[String]) -> Vec<Box<[char]>> {
    let mut m = HashMap::new();
    for item in a {
        m.insert(item.clone(), true);
    }
    for item in b {
        if !m.contains_key(item) {
            m.insert(item.clone(), true);
        }
    }

    let mut out: Vec<Box<[char]>> = Vec::with_capacity(m.len());
    for word in m.keys() {
        out.push(word.chars().collect::<Vec<_>>().into_boxed_slice());
    }
    out
}
