
use std::cmp::max;

fn jaro_winkler_similarity(str1: &str, str2: &str) -> f32 {
    let jaro_sim = jaro_similarity(str1, str2);

    if jaro_sim != 0.0 && jaro_sim != 1.0 {
        let rune_str1: Vec<char> = str1.chars().collect();
        let rune_str2: Vec<char> = str2.chars().collect();

        let rune_str1len = rune_str1.len();
        let rune_str2len = rune_str2.len();

        let mut prefix = 0;

        for i in 0..std::cmp::min(rune_str1len, rune_str2len) {
            if rune_str1[i] == rune_str2[i] {
                prefix += 1;
            } else {
                break;
            }
        }

        prefix = std::cmp::min(prefix, 4);

        return jaro_sim + 0.1 * prefix as f32 * (1.0 - jaro_sim);
    }

    return jaro_sim;
}

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
    let max_dist = (rune_str1len as isize + rune_str2len as isize) / 2 - 1;
    let mut str1_table: Vec<i32> = vec![0; rune_str1len];
    let mut str2_table: Vec<i32> = vec![0; rune_str2len];

    for i in 0..rune_str1len {
        for j in max(0, i as isize - max_dist)..std::cmp::min(rune_str2len as isize, i as isize + max_dist + 1) {
            if rune_str1[i] == rune_str2[j as usize] && str2_table[j as usize] == 0 {
                str1_table[i] = 1;
                str2_table[j as usize] = 1;
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

    return (match_count as f32 / rune_str1len as f32
        + match_count as f32 / rune_str2len as f32
        + (match_count as f32 - t) / match_count as f32)
        / 3.0;
}

fn equal(a: &[char], b: &[char]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    for i in 0..a.len() {
        if a[i] != b[i] {
            return false;
        }
    }
    return true;
}
