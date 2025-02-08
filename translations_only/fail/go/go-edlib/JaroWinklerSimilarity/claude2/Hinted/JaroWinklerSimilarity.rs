
use std::cmp::{max, min};

fn jaro_winkler_similarity(str1: &str, str2: &str) -> f32 {
    let jaro_sim = jaro_similarity(str1, str2);

    if jaro_sim != 0.0 && jaro_sim != 1.0 {
        let mut prefix = 0;
        let prefix_limit = 4;

        for i in 0..min(str1.len(), str2.len()) {
            if str1.as_bytes()[i] == str2.as_bytes()[i] {
                prefix += 1;
            } else {
                break;
            }
        }

        prefix = min(prefix, prefix_limit);

        jaro_sim + 0.1 * (prefix as f32) * (1.0 - jaro_sim)
    } else {
        jaro_sim
    }
}

fn jaro_similarity(str1: &str, str2: &str) -> f32 {
    if str1.len() == 0 || str2.len() == 0 {
        return 0.0;
    }

    if str1 == str2 {
        return 1.0;
    }

    let str1_len = str1.len();
    let str2_len = str2.len();

    let max_dist = max(str1_len, str2_len) / 2 - 1;

    let mut match_count = 0;
    let mut str1_matches = vec![0; str1_len];
    let mut str2_matches = vec![0; str2_len];

    for i in 0..str1_len {
        let start = max(0, i as isize - max_dist as isize) as usize;
        let end = min(str2_len, (i + max_dist + 1) as usize);

        for j in start..end {
            if str1.as_bytes()[i] == str2.as_bytes()[j] && str2_matches[j] == 0 {
                str1_matches[i] = 1;
                str2_matches[j] = 1;
                match_count += 1;
                break;
            }
        }
    }

    if match_count == 0 {
        return 0.0;
    }

    let mut transpositions = 0;
    let mut p = 0;

    for i in 0..str1_len {
        if str1_matches[i] == 1 {
            while str2_matches[p] == 0 {
                p += 1;
            }

            if str1.as_bytes()[i] != str2.as_bytes()[p] {
                transpositions += 1;
            }

            p += 1;
        }
    }

    transpositions /= 2;

    (match_count as f32 / str1_len as f32
        + match_count as f32 / str2_len as f32
        + (match_count as f32 - transpositions as f32) / match_count as f32)
        / 3.0
}
