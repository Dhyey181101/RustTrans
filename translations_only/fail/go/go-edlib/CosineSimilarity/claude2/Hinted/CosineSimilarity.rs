

use std::collections::HashMap;

fn cosine_similarity(str1: &str, str2: &str, split_length: isize) -> f32 {
    if str1.is_empty() || str2.is_empty() {
        return 0.0;
    }

    // Split string before rune conversion for cosine calculation
    // If split_length == 0 then split on whitespaces 
    // Else use shingle algorithm
    let mut splitted_str1: Vec<String> = Vec::new();
    let mut splitted_str2: Vec<String> = Vec::new();
    if split_length == 0 {
        splitted_str1 = str1.split(" ").map(|s| s.to_owned()).collect();
        splitted_str2 = str2.split(" ").map(|s| s.to_owned()).collect();
    } else {
        splitted_str1 = shingle_slice(str1, split_length as usize);
        splitted_str2 = shingle_slice(str2, split_length as usize);
    }

    // Conversion of splitted string into rune array
    let rune_str1: Vec<Vec<char>> = splitted_str1.iter().map(|s| s.chars().collect()).collect();
    let rune_str2: Vec<Vec<char>> = splitted_str2.iter().map(|s| s.chars().collect()).collect();

    // Create union keywords slice between input strings
    let union_str = union(&splitted_str1, &splitted_str2);
    
    let mut l1: Vec<isize> = vec![0; union_str.len()];
    let mut l2: Vec<isize> = vec![0; union_str.len()];

    for (i, word) in union_str.iter().enumerate() {
        let fw = find(&rune_str1, word);
        if fw != -1 {
            l1[i] = 1;
        }
        
        let fw = find(&rune_str2, word);
        if fw != -1 {
            l2[i] = 1; 
        }
    }

    // Compute cosine algorithm
    let mut cosine_sim: f32 = 0.0;
    for i in 0..union_str.len() {
        cosine_sim += (l1[i] * l2[i]) as f32;
    }
    
    return cosine_sim / ((sum(&l1) as f32 * sum(&l2) as f32).sqrt());
}

fn shingle_slice(s: &str, k: usize) -> Vec<String> {
    let mut out = Vec::new();
    if !s.is_empty() && k != 0 {
        let rune_s: Vec<char> = s.chars().collect();
        for i in 0..rune_s.len()-k+1 {
            let slice = &rune_s[i..i+k];
            let mut inserted = false;
            for v in &mut out {
                if *v == slice.iter().collect::<String>() {
                    inserted = true;
                    break;
                }
            }
            if !inserted {
                out.push(slice.iter().collect::<String>());
            }
        }
    }
    out
}

fn union(a: &[String], b: &[String]) -> Vec<Vec<char>> {
    let mut out: Vec<Vec<char>> = Vec::new();
    let mut inserted = HashMap::new();
    for item in a.iter() {
        if !inserted.contains_key(item) {
            out.push(item.chars().collect());
            inserted.insert(item, true);
        }
    }
    for item in b.iter() {
        if !inserted.contains_key(item) {
            out.push(item.chars().collect());
            inserted.insert(item, true);   
        }
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
    a.iter().zip(b).all(|(x, y)| x == y)
}

fn sum(arr: &[isize]) -> isize {
    arr.iter().sum()  
}

