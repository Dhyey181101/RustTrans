
fn jaro_similarity(str1: &str, str2: &str) -> f32 {
    let rune_str1: Vec<char> = str1.chars().collect();
    let rune_str2: Vec<char> = str2.chars().collect();

    let rune_str1len = rune_str1.len();
    let rune_str2len = rune_str2.len();
    if rune_str1len == 0 || rune_str2len == 0 {
        return 0.0;
    } else if equal(&rune_str1, &rune_str2) {
        return 1.0;
    }

    let mut match_count = 0;
    let max_dist = max(rune_str1len as isize, rune_str2len as isize) / 2 - 1;
    let mut str1_table = vec![0; rune_str1len];
    let mut str2_table = vec![0; rune_str2len];

    for i in 0..rune_str1len {
        let start = max(0, i as isize - max_dist) as usize;
        let end = min(rune_str2len, (i as isize + max_dist + 1) as usize);
        for j in start..end {
            if rune_str1[i] == rune_str2[j] && str2_table[j] == 0 {
                str1_table[i] = 1;
                str2_table[j] = 1;
                match_count += 1;
                break;
            }
        }
    }
    if match_count == 0 {
        return 0.0;
    }

    let mut t = 0.0;
    let mut p = 0;
    for i in 0..rune_str1len {
        if str1_table[i] == 1 {
            while str2_table[p] == 0 {
                p += 1;
            }
            if rune_str1[i] != rune_str2[p] {
                t += 1.0;
            }
            p += 1;
        }
    }
    t /= 2.0;

    (match_count as f32 / rune_str1len as f32 +
     match_count as f32 / rune_str2len as f32 +
     (match_count as f32 - t) / match_count as f32) / 3.0
}

fn equal(a: &[char], b: &[char]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    for (i, &v) in a.iter().enumerate() {
        if v != b[i] {
            return false;
        }
    }
    true
}

fn max(a: isize, b: isize) -> isize {
    if b > a { b } else { a }
}

fn min(a: usize, b: usize) -> usize {
    if b < a { b } else { a }
}
