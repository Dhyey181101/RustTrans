
use std::collections::{HashMap, HashSet};

fn cosine_similarity(str1: &str, str2: &str, split_length: isize) -> f32 {
    if str1.is_empty() || str2.is_empty() {
        return 0.0;
    }

    let (splitted_str1, splitted_str2) = if split_length == 0 {
        (str1.split_whitespace().map(String::from).collect(), str2.split_whitespace().map(String::from).collect())
    } else {
        (shingle_slice(str1, split_length), shingle_slice(str2, split_length))
    };

    let rune_str1: Vec<Vec<char>> = splitted_str1.iter().map(|s| s.chars().collect()).collect();
    let rune_str2: Vec<Vec<char>> = splitted_str2.iter().map(|s| s.chars().collect()).collect();

    let union_str = union(&splitted_str1, &splitted_str2);
    let mut l1 = vec![0; union_str.len()];
    let mut l2 = vec![0; union_str.len()];
    for (i, word) in union_str.iter().enumerate() {
        l1[i] = find(&rune_str1, word).is_some() as isize;
        l2[i] = find(&rune_str2, word).is_some() as isize;
    }

    let mut cosine_sim = 0.0;
    for i in 0..union_str.len() {
        cosine_sim += (l1[i] as f32) * (l2[i] as f32);
    }

    cosine_sim / (sum_isize(&l1) as f32).sqrt() * (sum_isize(&l2) as f32).sqrt()
}

fn shingle_slice(s: &str, k: isize) -> Vec<String> {
    let mut res: HashMap<String, i32> = HashMap::new();
    if !s.is_empty() && k != 0 {
        let chars: Vec<char> = s.chars().collect();
        for i in 0..(chars.len() - k as usize + 1) {
            *res.entry(chars[i..(i + k as usize)].iter().collect::<String>()).or_insert(0) += 1;
        }
    }
    res.into_keys().map(String::from).collect()
}

fn union(a: &[String], b: &[String]) -> Vec<Vec<char>> {
    let mut set: HashSet<String> = a.iter().cloned().collect();
    let mut union = a.to_vec();
    b.iter().filter(|x| set.insert(x.to_string())).for_each(|x| union.push(x.clone()));
    union.iter().map(|x| x.chars().collect()).collect()  
}

fn find(slice: &[Vec<char>], val: &[char]) -> Option<usize> {
    slice.iter().position(|item| equal(item, val))
}

fn equal(a: &[char], b: &[char]) -> bool {
    a.len() == b.len() && a.iter().zip(b).all(|(x, y)| x == y)
}

fn sum_isize(arr: &[isize]) -> isize {
    arr.iter().sum()
}

