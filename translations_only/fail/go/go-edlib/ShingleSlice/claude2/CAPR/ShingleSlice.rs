
fn shingle_slice(s: &str, k: isize) -> Vec<String> {
    let mut out = Vec::new();
    let mut m = std::collections::HashMap::new();
    if !s.is_empty() && k != 0 {
        let rune_s = s.as_bytes();
        for i in 0..rune_s.len()-k as usize+1 {
            *m.entry(std::str::from_utf8(&rune_s[i..i+k as usize]).unwrap().to_string()).or_insert(0) += 1;
        }
        for k in m.keys() {
            out.push(k.clone());
        }
    }
    out
}
