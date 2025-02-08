
use std::cmp::max as std_max;
use std::cmp::min as other_min;

fn jaro_winkler_similarity(str1: &str, str2: &str) -> f32 {
    let jaro_sim = jaro_similarity(str1, str2);

    if jaro_sim != 0.0 && jaro_sim != 1.0 {
        let rune_str1 = str1.as_bytes();
        let rune_str2 = str2.as_bytes();

        let rune_str1_len = rune_str1.len();
        let rune_str2_len = rune_str2.len();

        let mut prefix = 0;

        for i in 0..other_min(rune_str1_len, rune_str2_len) {
            if rune_str1[i] == rune_str2[i] {
                prefix += 1;
            } else {
                break;
            }
        }

        prefix = other_min(prefix, 4);

        jaro_sim + 0.1 * (prefix as f32) * (1.0 - jaro_sim)
    } else {
        jaro_sim
    }
}

fn jaro_similarity(str1: &str, str2: &str) -> f32 {
    let rune_str1 = str1.as_bytes();
    let rune_str2 = str2.as_bytes();

    let rune_str1_len = rune_str1.len();
    let rune_str2_len = rune_str2.len();

    if rune_str1_len == 0 || rune_str2_len == 0 {
        return 0.0;
    } else if rune_str1 == rune_str2 {
        return 1.0;
    }

    let max_dist = std_max(rune_str1_len, rune_str2_len) / 2 - 1;
    let mut str1_table = vec![0; rune_str1_len];
    let mut str2_table = vec![0; rune_str2_len];

    let mut match_count = 0;

    for i in 0..rune_str1_len {
        let start = if i > max_dist {i - max_dist} else {0};
        let end = other_min(rune_str2_len, i + max_dist + 1);
        
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

    for i in 0..rune_str1_len {
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

    (match_count as f32 / rune_str1_len as f32
        + match_count as f32 / rune_str2_len as f32
        + (match_count as f32 - t) / match_count as f32) 
        / 3.0
}

