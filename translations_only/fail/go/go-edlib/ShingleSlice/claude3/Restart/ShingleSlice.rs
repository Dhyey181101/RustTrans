
use std::collections::HashMap;

fn shingle_slice(s: &str, k: isize) -> Vec<String> {
    let mut out = Vec::new();
    let mut m = HashMap::new();

    if !s.is_empty() && k != 0 {
        let mut chars = s.chars();
        let mut window = chars.by_ref().take(k as usize).collect::<String>();

        m.insert(window.clone(), 1);

        for c in chars {
            window.remove(0);
            window.push(c);
            *m.entry(window.clone()).or_insert(0) += 1;
        }

        for (k, _) in m {
            out.push(k);
        }
    }

    out
}
