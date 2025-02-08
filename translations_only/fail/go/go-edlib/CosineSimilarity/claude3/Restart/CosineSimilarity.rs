

use std::collections::HashMap;

fn cosine_similarity(str1: &str, str2: &str, split_length: isize) -> f32 {
    if str1.is_empty() || str2.is_empty() {
        return 0.0;
    }

    let splitted_str1: Vec<String> = if split_length == 0 {
        str1.split_whitespace().map(String::from).collect()
    } else {
        shingle_slice(str1, split_length as usize)
    };

    let splitted_str2: Vec<String> = if split_length == 0 {
        str2.split_whitespace().map(String::from).collect()
    } else {
        shingle_slice(str2, split_length as usize)
    };

    let rune_str1: Vec<Vec<char>> = splitted_str1.iter().map(|s| s.chars().collect()).collect();
    let rune_str2: Vec<Vec<char>> = splitted_str2.iter().map(|s| s.chars().collect()).collect();

    let mut l1: Vec<isize> = Vec::new();
    let mut l2: Vec<isize> = Vec::new();

    let union_str: Vec<Vec<char>> = union(&splitted_str1, &splitted_str2);

    for word in &union_str {
        let fw = find(&rune_str1, word);
        if fw != -1 {
            l1.push(1);
        } else {
            l1.push(0);
        }

        let fw = find(&rune_str2, word);
        if fw != -1 {
            l2.push(1);
        } else {
            l2.push(0);
        }
    }

    let mut cosine_sim: f32 = 0.0;
    for i in 0..union_str.len() {
        cosine_sim += (l1[i] * l2[i]) as f32;
    }

    let sum1: f32 = sum(&l1) as f32;
    let sum2: f32 = sum(&l2) as f32;
    cosine_sim / (sum1 * sum2).sqrt()
}

fn shingle_slice(s: &str, k: usize) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    let mut m: HashMap<String, isize> = HashMap::new();

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
    let mut m: HashMap<String, bool> = HashMap::new();
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

fn find(slice: &[Vec<char>], val: &[char]) -> isize {
    for (i, item) in slice.iter().enumerate() {
        if equal(item, val) {
            return i as isize;
        }
    }
    -1
}

fn equal(a: &[char], b: &[char]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    for (i, v) in a.iter().enumerate() {
        if v != &b[i] {
            return false;
        }
    }
    true
}

fn sum(arr: &[isize]) -> isize {
    let mut res: isize = 0;
    for v in arr {
        res += *v;
    }
    res
}

