
use std::cmp;

fn jaro_winkler_similarity(str1: &str, str2: &str) -> f32 {
    // Get Jaro similarity index between str1 and str2
    let jaro_sim = jaro_similarity(str1, str2);

    if jaro_sim != 0.0 && jaro_sim != 1.0 {
        // Convert string parameters to char vectors to be compatible with non-ASCII
        let char_str1: Vec<char> = str1.chars().collect();
        let char_str2: Vec<char> = str2.chars().collect();

        // Get and store length of these strings
        let char_str1_len = char_str1.len();
        let char_str2_len = char_str2.len();

        let mut prefix = 0;

        // Find length of the common prefix
        for i in 0..cmp::min(char_str1_len, char_str2_len) {
            if char_str1[i] == char_str2[i] {
                prefix += 1;
            } else {
                break;
            }
        }

        // Normalized prefix count with Winkler's constraint
        // (prefix length must be inferior or equal to 4)
        let prefix = cmp::min(prefix, 4);

        // Return calculated Jaro-Winkler similarity index
        return jaro_sim + 0.1 * prefix as f32 * (1.0 - jaro_sim);
    }

    jaro_sim
}

fn jaro_similarity(str1: &str, str2: &str) -> f32 {
    // Convert string parameters to char vectors to be compatible with non-ASCII
    let char_str1: Vec<char> = str1.chars().collect();
    let char_str2: Vec<char> = str2.chars().collect();

    // Get and store length of these strings
    let char_str1_len = char_str1.len();
    let char_str2_len = char_str2.len();
    if char_str1_len == 0 || char_str2_len == 0 {
        return 0.0;
    } else if char_str1 == char_str2 {
        return 1.0;
    }

    let mut match_count = 0;
    // Maximum matching distance allowed
    let max_dist = cmp::max(char_str1_len, char_str2_len) / 2 - 1;
    // Correspondence tables (1 for matching and 0 if it's not the case)
    let mut str1_table = vec![0; char_str1_len];
    let mut str2_table = vec![0; char_str2_len];

    // Check for matching characters in both strings
    for i in 0..char_str1_len {
        for j in cmp::max(0, i as isize - max_dist as isize) as usize
            ..cmp::min(char_str2_len, i + max_dist + 1)
        {
            if char_str1[i] == char_str2[j] && str2_table[j] == 0 {
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
    for i in 0..char_str1_len {
        if str1_table[i] == 1 {
            while str2_table[p] == 0 {
                p += 1;
            }
            if char_str1[i] != char_str2[p] {
                t += 1.0;
            }
            p += 1;
        }
    }
    t /= 2.0;

    (match_count as f32 / char_str1_len as f32
        + match_count as f32 / char_str2_len as f32
        + (match_count as f32 - t) / match_count as f32)
        / 3.0
}
