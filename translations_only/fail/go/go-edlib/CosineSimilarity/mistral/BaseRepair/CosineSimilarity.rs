
use std::collections::HashMap;
use std::iter;
use std::ops::Mul;
use std::str;

const EPSILON: f32 = 1e-9;

fn cosine_similarity(str1: &str, str2: &str, split_length: usize) -> f32 {
    if str1.is_empty() || str2.is_empty() {
        0.0
    } else {
        let mut vector1 = HashMap::new();
        let mut vector2 = HashMap::new();

        for i in str1.as_bytes().chunks(split_length) {
            let key = str::from_utf8(i).unwrap();
            *vector1.entry(key).or_insert(0) += 1;
        }

        for i in str2.as_bytes().chunks(split_length) {
            let key = str::from_utf8(i).unwrap();
            *vector2.entry(key).or_insert(0) += 1;
        }

        let dot_product = vector1
            .iter()
            .filter_map(|(&k, &v1)| vector2.get(&k).map(|&v2| v1 as f32 * v2 as f32))
            .sum::<f32>();

        let magnitude1 = (vector1.values().sum::<usize>() as f32).sqrt();
        let magnitude2 = (vector2.values().sum::<usize>() as f32).sqrt();

        if magnitude1.abs() < EPSILON || magnitude2.abs() < EPSILON {
            0.0
        } else {
            dot_product / (magnitude1 * magnitude2)
        }
    }
}
