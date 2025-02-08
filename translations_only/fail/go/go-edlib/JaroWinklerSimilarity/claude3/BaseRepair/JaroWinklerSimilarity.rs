
use std::cmp::{max, min};

fn jaro_winkler_similarity(str1: &str, str2: &str) -> f32 {
    // Get Jaro similarity index between str1 and str2
    let jaro_sim = jaro_similarity(str1, str2);

    if jaro_sim != 0.0 && jaro_sim != 1.0 {
        // Convert string parameters to char vectors to be compatible with non-ASCII
        let mut str1_chars: Vec<char> = str1.chars().collect();
        let mut str2_chars: Vec<char> = str2.chars().collect();

        // Get and store length of these strings
        let str1_len = str1_chars.len();
        let str2_len = str2_chars.len();

        let mut prefix = 0;

        // Find length of the common prefix
        for i in 0..min(str1_len, str2_len) {
            if str1_chars[i] == str2_chars[i] {
                prefix += 1;
            } else {
                break;
            }
        }

        // Normalized prefix count with Winkler's constraint
        // (prefix length must be inferior or equal to 4)
        let prefix = min(prefix, 4);

        // Return calculated Jaro-Winkler similarity index
        return jaro_sim + 0.1 * prefix as f32 * (1.0 - jaro_sim);
    }

    jaro_sim
}

fn jaro_similarity(str1: &str, str2: &str) -> f32 {
    // Convert string parameters to char vectors to be compatible with non-ASCII
    let mut str1_chars: Vec<char> = str1.chars().collect();
    let mut str2_chars: Vec<char> = str2.chars().collect();

    // Get and store length of these strings
    let str1_len = str1_chars.len();
    let str2_len = str2_chars.len();
    if str1_len == 0 || str2_len == 0 {
        return 0.0;
    } else if str1_chars == str2_chars {
        return 1.0;
    }

    let mut match_count = 0;
    // Maximum matching distance allowed
    let max_dist = max(str1_len, str2_len) / 2 - 1;
    // Correspondence tables (1 for matching and 0 if it's not the case)
    let mut str1_table = vec![0; str1_len];
    let mut str2_table = vec![0; str2_len];

    // Check for matching characters in both strings
    for i in 0..str1_len {
        for j in max(0, i as isize - max_dist as isize) as usize..min(str2_len, i + max_dist + 1) {
            if str1_chars[i] == str2_chars[j] && str2_table[j] == 0 {
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
    // Check for possible translations
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
