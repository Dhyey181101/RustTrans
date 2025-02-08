
use std::collections::HashMap;

fn sorensen_dice_coefficient(str1: &str, str2: &str, split_length: isize) -> f32 {
    if str1.is_empty() && str2.is_empty() {
        return 0.0;
    }
    let shingle1 = shingle(str1, split_length as usize);
    let shingle2 = shingle(str2, split_length as usize);

    let mut intersection = 0.0;
    for (k, _) in shingle1.iter() {
        if shingle2.contains_key(k) {
            intersection += 1.0;
        }
    }
    2.0 * intersection / (shingle1.len() as f32 + shingle2.len() as f32)
}

fn shingle(s: &str, k: usize) -> HashMap<Box<str>, isize> {
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
        let mut window_str = String::from_iter(window.iter().cloned());
        m.insert(window_str.into_boxed_str(), 1);
        for c in chars {
            window.remove(0);
            window.push(c);
            window_str = String::from_iter(window.iter().cloned());
            *m.entry(window_str.into_boxed_str()).or_insert(0) += 1;
        }
    }
    m
}
