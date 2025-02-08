
use std::collections::HashMap;
use std::isize;

fn Shingle(s: &str, k: isize) -> HashMap<String, i32> {
    let mut m = HashMap::new();
    if s != "" && k != 0 {
        let rune_s: Vec<char> = s.chars().collect();

        for i in 0..(rune_s.len() as isize - k + 1) {
            let key = rune_s[i as usize..(i as isize + k) as usize].iter().collect::<String>();
            *m.entry(key).or_insert(0) += 1;
        }
    }
    m
}
