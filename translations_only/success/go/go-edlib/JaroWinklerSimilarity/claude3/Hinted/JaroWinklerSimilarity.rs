

fn jaro_winkler_similarity(str1: &str, str2: &str) -> f32 {
    let jaro_sim = jaro_similarity(str1, str2);

    if jaro_sim != 0.0 && jaro_sim != 1.0 {
        let mut char_vec1: Vec<char> = str1.chars().collect();
        let mut char_vec2: Vec<char> = str2.chars().collect();

        let char_vec1_len = char_vec1.len();
        let char_vec2_len = char_vec2.len();

        let mut prefix = 0;

        for i in 0..usize::min(char_vec1_len, char_vec2_len) {
            if char_vec1[i] == char_vec2[i] {
                prefix += 1;
            } else {
                break;
            }
        }

        let prefix = isize::min(prefix as isize, 4);

        return jaro_sim + 0.1 * (prefix as f32) * (1.0 - jaro_sim);
    }

    jaro_sim
}

fn jaro_similarity(str1: &str, str2: &str) -> f32 {
    let mut char_vec1: Vec<char> = str1.chars().collect();
    let mut char_vec2: Vec<char> = str2.chars().collect();

    let char_vec1_len = char_vec1.len();
    let char_vec2_len = char_vec2.len();
    if char_vec1_len == 0 || char_vec2_len == 0 {
        return 0.0;
    } else if char_vec1 == char_vec2 {
        return 1.0;
    }

    let mut match_count = 0;
    let max_dist = isize::max(char_vec1_len.try_into().unwrap(), char_vec2_len.try_into().unwrap()) / 2 - 1;
    let mut str1_table = vec![0; char_vec1_len];
    let mut str2_table = vec![0; char_vec2_len];

    for i in 0..char_vec1_len {
        for j in isize::max(0, (i as isize) - max_dist)..isize::min(char_vec2_len as isize, (i as isize) + max_dist + 1) {
            if char_vec1[i] == char_vec2[j as usize] && str2_table[j as usize] == 0 {
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
    for i in 0..char_vec1_len {
        if str1_table[i] == 1 {
            while str2_table[p] == 0 {
                p += 1;
            }
            if char_vec1[i] != char_vec2[p] {
                t += 1.0;
            }
            p += 1;
        }
    }
    t /= 2.0;

    (match_count as f32 / char_vec1_len as f32
        + match_count as f32 / char_vec2_len as f32
        + (match_count as f32 - t) / match_count as f32)
        / 3.0
}

