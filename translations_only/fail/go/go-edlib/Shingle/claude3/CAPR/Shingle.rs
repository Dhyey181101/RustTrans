
use std::collections::HashMap;

fn Shingle(s: &str, k: isize) -> HashMap<String, isize> {
    let mut m = HashMap::new();
    if !s.is_empty() && k != 0 {
        let mut chars = s.chars();

        let mut window = chars.by_ref().take(k as usize).collect::<Vec<_>>();
        m.insert(window.iter().collect::<String>(), 1);

        for next in chars {
            window.remove(0);
            window.push(next);
            let key: String = window.iter().collect();
            *m.entry(key).or_insert(0) += 1;
        }
    }
    m
}
