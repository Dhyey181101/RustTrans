
use std::collections::HashMap;

fn jaccard_similarity(str1: &str, str2: &str, split_length: isize) -> Option<f32> {
    if str1.is_empty() || str2.is_empty() {
        return Some(0.0);
    }

    // Split string before rune conversion for jaccard calculation
    // If split_length == 0 then split on whitespaces 
    // Else use shingle algorithm
    let mut splitted_str1: Vec<String> = Vec::new();
    let mut splitted_str2: Vec<String> = Vec::new();
    if split_length == 0 {
        splitted_str1 = str1.split(" ").map(|s| s.to_string()).collect();
        splitted_str2 = str2.split(" ").map(|s| s.to_string()).collect();
    } else {
        splitted_str1 = shingle_slice(str1, split_length as usize);
        splitted_str2 = shingle_slice(str2, split_length as usize);
    };

    // Conversion of splitted string into rune array
    let rune_str1: Vec<Vec<char>> = splitted_str1.iter().map(|s| s.chars().collect()).collect();
    let rune_str2: Vec<Vec<char>> = splitted_str2.iter().map(|s| s.chars().collect()).collect();

    // Create union keywords slice between input strings
    let union_str = union(&splitted_str1, &splitted_str2);
    
    let jacc = (rune_str1.len() + rune_str2.len() - union_str.len()) as f32;

    Some(jacc / union_str.len() as f32)
}

fn shingle_slice(s: &str, k: usize) -> Vec<String> {
    let mut out = Vec::new();
    let mut m = HashMap::new();
    if !s.is_empty() && k != 0 {
        let rune_s: Vec<char> = s.chars().collect();
        for i in 0..rune_s.len()-k+1 {
            *m.entry(rune_s[i..i+k].iter().collect::<String>()).or_insert(0) += 1;
        }
        for (k,_) in m {
            out.push(k);
        }
    }
    out 
}

fn union(a: &[String], b: &[String]) -> Vec<Vec<char>> {
    let mut m = HashMap::new();
    for item in a.iter() {
        m.insert(item.clone(), true);
    }
    let mut a_copy = a.to_vec();
    for item in b {
        if !m.contains_key(item) {
             a_copy.push(item.clone());
        }
    }

    a_copy.iter().map(|word| word.chars().collect()).collect()
}
