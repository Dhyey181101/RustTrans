
fn jaro_similarity(str1: &str, str2: &str) -> f32 {
    let mut str1_chars: Vec<char> = str1.chars().collect();
    let mut str2_chars: Vec<char> = str2.chars().collect();

    let str1_len = str1_chars.len();
    let str2_len = str2_chars.len();
    if str1_len == 0 || str2_len == 0 {
        return 0.0;
    } else if str1_chars == str2_chars {
        return 1.0;
    }

    let mut r#match = 0;
    let max_dist = isize::max(str1_len as isize, str2_len as isize) / 2 - 1;
    let mut str1_table = vec![0; str1_len];
    let mut str2_table = vec![0; str2_len];

    for i in 0..str1_len {
        for j in isize::max(0, i as isize - max_dist)..isize::min(str2_len as isize, i as isize + max_dist + 1) {
            let j = j as usize;
            if str1_chars[i] == str2_chars[j] && str2_table[j] == 0 {
                str1_table[i] = 1;
                str2_table[j] = 1;
                r#match += 1;
                break;
            }
        }
    }
    if r#match == 0 {
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

    (r#match as f32 / str1_len as f32
        + r#match as f32 / str2_len as f32
        + (r#match as f32 - t) / r#match as f32)
        / 3.0
}
