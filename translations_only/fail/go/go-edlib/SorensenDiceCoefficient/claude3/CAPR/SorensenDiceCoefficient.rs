
use std::collections::HashMap;

fn sorensen_dice_coefficient(str1: &str, str2: &str, split_length: usize) -> f32 {
    if str1.is_empty() && str2.is_empty() {
        return 0.0;
    }

    let shingle1 = shingle(str1, split_length);
    let shingle2 = shingle(str2, split_length);

    let mut intersection: f32 = 0.0;
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

        while let Some(start) = chars.as_str().get(..k) {
            let key = start.to_owned().into_boxed_str();
            *m.entry(key).or_insert(0) += 1;
            chars.by_ref().nth(1);
        }
    }

    m
}
