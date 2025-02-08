

use std::collections::HashMap;
use std::iter;

fn sorensen_dice_coefficient(str1: &str, str2: &str, split_length: usize) -> f32 {
    if str1.is_empty() && str2.is_empty() {
        return 0.0;
    }

    let shingle1 = shingle(str1, split_length);
    let shingle2 = shingle(str2, split_length);

    let intersection = shingle1.keys().filter(|k| shingle2.contains_key(k.as_str())).count() as f32;
    (2.0 * intersection) / (shingle1.len() as f32 + shingle2.len() as f32)
}

fn shingle(s: &str, k: usize) -> HashMap<String, u32> {
    let mut m = HashMap::new();
    if !s.is_empty() && k != 0 {
        let rune_s: Vec<char> = s.chars().collect();

        for i in 0..(rune_s.len() - k + 1) {
            let key = rune_s[i..i + k].iter().collect::<String>();
            *m.entry(key).or_insert(0) += 1;
        }
    }
    m
}

