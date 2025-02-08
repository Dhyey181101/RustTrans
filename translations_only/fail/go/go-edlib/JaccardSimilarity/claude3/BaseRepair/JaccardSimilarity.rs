
use std::collections::HashSet;

fn jaccard_similarity(str1: &str, str2: &str, split_length: isize) -> f32 {
    if str1.is_empty() || str2.is_empty() {
        return 0.0;
    }

    let (splitted_str1, splitted_str2) = if split_length == 0 {
        (
            str1.split_whitespace().map(|s| s.to_string().into_boxed_str()).collect::<Vec<_>>(),
            str2.split_whitespace().map(|s| s.to_string().into_boxed_str()).collect::<Vec<_>>(),
        )
    } else {
        (
            shingle_slice(str1, split_length as usize),
            shingle_slice(str2, split_length as usize),
        )
    };

    let union_str = union(&splitted_str1, &splitted_str2);
    let jacc = (splitted_str1.len() + splitted_str2.len() - union_str.len()) as f32;

    jacc / union_str.len() as f32
}

fn shingle_slice(s: &str, k: usize) -> Vec<Box<str>> {
    let mut out = Vec::new();
    let mut m = HashSet::new();

    if !s.is_empty() && k != 0 {
        let rune_s: Vec<char> = s.chars().collect();
        for i in 0..rune_s.len() - k + 1 {
            let shingle: String = rune_s[i..i + k].iter().collect();
            m.insert(shingle);
        }
        out = m.into_iter().map(|s| s.into_boxed_str()).collect();
    }

    out
}

fn union(a: &[Box<str>], b: &[Box<str>]) -> Vec<Box<str>> {
    let mut m = HashSet::new();
    for item in a.iter().chain(b.iter()) {
        m.insert(item.clone());
    }

    m.into_iter().collect()
}
