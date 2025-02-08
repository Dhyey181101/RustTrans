
fn jaro_similarity(str1: &str, str2: &str) -> f32 {
    let rune_str1: Vec<char> = str1.chars().collect();
    let rune_str2: Vec<char> = str2.chars().collect();

    let rune_str1_len = rune_str1.len();
    let rune_str2_len = rune_str2.len();
    if rune_str1_len == 0 || rune_str2_len == 0 {
        return 0.0;
    } else if rune_str1 == rune_str2 {
        return 1.0;
    }

    let mut match_count = 0;
    let max_dist = (rune_str1_len.max(rune_str2_len) / 2) - 1;
    let mut str1_table: Vec<i32> = vec![0; rune_str1_len];
    let mut str2_table: Vec<i32> = vec![0; rune_str2_len];

    for i in 0..rune_str1_len {
        for j in (i.max(0) - max_dist)..(i + max_dist + 1).min(rune_str2_len) {
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

    let mut transpositions = 0.0;
    let mut point = 0;
    for i in 0..rune_str1_len {
        if str1_table[i] == 1 {
            while str2_table[point] == 0 {
                point += 1;
            }
            if rune_str1[i] != rune_str2[point] {
                transpositions += 1.0;
            }
            point += 1;
        }
    }
    transpositions /= 2.0;

    return (match_count as f32 / rune_str1_len as f32 + match_count as f32 / rune_str2_len as f32 + (match_count as f32 - transpositions) / match_count as f32) / 3.0;
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
    true
}

fn max(a: isize, b: isize) -> isize {
    if b > a {
        b
    } else {
        a
    }
}

fn min(a: isize, b: isize) -> isize {
    if b < a {
        b
    } else {
        a
    }
}
