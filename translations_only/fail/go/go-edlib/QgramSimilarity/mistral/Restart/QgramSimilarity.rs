

use std::collections::HashMap;
use std::iter;
use std::isize;

fn QgramSimilarity(str1: String, str2: String, split_length: isize) -> f32 {
    let splitted_str1 = Shingle(str1, split_length as usize);
    let splitted_str2 = Shingle(str2, split_length as usize);
    let res = QgramDistanceCustomNgram(&splitted_str1, &splitted_str2);
    1.0 - (res as f32 / (splitted_str1.len() as f32 + splitted_str2.len() as f32))
}

fn Shingle(s: String, k: usize) -> HashMap<String, isize> {
    let mut m = HashMap::new();
    if s != "" && k != 0 {
        let rune_s: Vec<char> = s.chars().collect();

        for i in 0..rune_s.len() - k + 1 {
            let ngram: String = rune_s[i..i + k].iter().collect();
            *m.entry(ngram).or_insert(0) += 1;
        }
    }
    m
}

fn QgramDistanceCustomNgram(splitted_str1: &HashMap<String, isize>, splitted_str2: &HashMap<String, isize>) -> isize {
    let mut union = HashMap::new();
    for (k, _) in splitted_str1 {
        union.insert(k.clone(), 0);
    }
    for (k, _) in splitted_str2 {
        union.insert(k.clone(), 0);
    }

    let res: isize = union.iter().fold(0, |acc, (k, _)| {
        acc + isize::abs(splitted_str1[k] as isize - splitted_str2[k] as isize)
    });

    res
}

