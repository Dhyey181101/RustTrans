
fn jaro_similarity(str1: &str, str2: &str) -> f32 {
    let mut str1_chars: Vec<char> = str1.chars().collect();
    let mut str2_chars: Vec<char> = str2.chars().collect();

    let str1_len = str1_chars.len();
    let str2_len = str2_chars.len();
    if str1_len == 0 || str2_len == 0 {
        return 0.0;
    } else if equal(&str1_chars, &str2_chars) {
        return 1.0;
    }

    let mut match_count = 0;
    let max_dist = max(str1_len, str2_len) / 2;
    let mut str1_table: Vec<isize> = vec![0; str1_len];
    let mut str2_table: Vec<isize> = vec![0; str2_len];

    for i in 0..str1_len {
        for j in 0..str2_len {
            let j_isize = j as isize;
            if (j_isize as usize >= max(i, max_dist) && j_isize as usize <= i + max_dist)
                && str1_chars[i] == str2_chars[j]
                && str2_table[j] == 0
            {
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
    for i in 0..str1_len {
        if str1_table[i] == 1 {
            while str2_table[p] == 0 {
                p += 1;
            }
            if str1_chars[i] != str2_chars[p] {
                t += 1.0;
            }
            p += 1;
        }
    }
    t /= 2.0;

    (match_count as f32 / str1_len as f32
        + match_count as f32 / str2_len as f32
        + (match_count as f32 - t) / match_count as f32)
        / 3.0
}

fn equal(a: &[char], b: &[char]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    for (i, v) in a.iter().enumerate() {
        if v != &b[i] {
            return false;
        }
    }
    true
}

fn max(a: usize, b: usize) -> usize {
    if b > a {
        b
    } else {
        a
    }
}
