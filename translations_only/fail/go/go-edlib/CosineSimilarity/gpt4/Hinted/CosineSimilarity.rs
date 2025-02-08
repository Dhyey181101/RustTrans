
use std::collections::HashSet;
use std::collections::HashMap;

fn cosine_similarity(str1: &str, str2: &str, split_length: isize) -> f32 {
    if str1.is_empty() || str2.is_empty() {
        return 0.0;
    }

    let splitted_str1: Vec<&str>;
    let splitted_str2: Vec<&str>;
    if split_length == 0 {
        splitted_str1 = str1.split_whitespace().collect();
        splitted_str2 = str2.split_whitespace().collect();
    } else {
        splitted_str1 = shingle_slice(str1, split_length as usize);
        splitted_str2 = shingle_slice(str2, split_length as usize);
    }

    let rune_str1: Vec<Vec<char>> = splitted_str1.iter().map(|&s| s.chars().collect()).collect();
    let rune_str2: Vec<Vec<char>> = splitted_str2.iter().map(|&s| s.chars().collect()).collect();

    let union_str = union(&splitted_str1, &splitted_str2);
    let mut l1: Vec<isize> = Vec::new();
    let mut l2: Vec<isize> = Vec::new();

    for word in union_str.iter() {
        let fw = find(&rune_str1, word);
        l1.push(if fw != -1 { 1 } else { 0 });

        let fw = find(&rune_str2, word);
        l2.push(if fw != -1 { 1 } else { 0 });
    }

    let mut cosine_sim: f32 = 0.0;
    for i in 0..union_str.len() {
        cosine_sim += (l1[i] * l2[i]) as f32;
    }

    cosine_sim / ((sum(&l1) * sum(&l2)) as f64).sqrt() as f32
}

fn shingle_slice(s: &str, k: usize) -> Vec<&str> {
    let mut out: Vec<&str> = Vec::new();
    let mut m: HashMap<&str, isize> = HashMap::new();
    if !s.is_empty() && k != 0 {
        let rune_s: Vec<char> = s.chars().collect();
        for i in 0..=rune_s.len().saturating_sub(k) {
            let shingle: &str = &s[i..i + k];
            *m.entry(shingle).or_insert(0) += 1;
        }
        for k in m.keys() {
            out.push(k);
        }
    }
    out
}

fn union(a: &Vec<&str>, b: &Vec<&str>) -> Vec<Vec<char>> {
    let mut set: HashSet<&str> = HashSet::new();
    let mut out: Vec<Vec<char>> = Vec::new();
    for &item in a.iter().chain(b.iter()) {
        if set.insert(item) {
            out.push(item.chars().collect());
        }
    }
    out
}

fn find(slice: &Vec<Vec<char>>, val: &Vec<char>) -> isize {
    for (i, item) in slice.iter().enumerate() {
        if item == val {
            return i as isize;
        }
    }
    -1
}

fn equal(a: &Vec<char>, b: &Vec<char>) -> bool {
    a == b
}

fn sum(arr: &Vec<isize>) -> isize {
    arr.iter().sum()
}
