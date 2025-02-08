
use std::collections::HashMap;

fn sorensen_dice_coefficient(str1: &str, str2: &str, split_length: isize) -> f32 {
    if str1.is_empty() && str2.is_empty() {
        return 0.0;
    }

    let shingle1 = shingle(str1, split_length);
    let shingle2 = shingle(str2, split_length);

    let mut intersection: f32 = 0.0;
    for i in shingle1.keys() {
        if shingle2.contains_key(i) {
            intersection += 1.0;
        }
    }

    return 2.0 * intersection / (shingle1.len() + shingle2.len()) as f32;
}

fn shingle(s: &str, k: isize) -> HashMap<String, isize> {
    let mut m = HashMap::new();
    if !s.is_empty() && k != 0 {
        let rune_s: Vec<char> = s.chars().collect();

        for i in 0..(rune_s.len() - k as usize + 1) {
            *m.entry(rune_s[i..(i + k as usize)].iter().collect()).or_insert(0) += 1;
        }
    }
    m
}
