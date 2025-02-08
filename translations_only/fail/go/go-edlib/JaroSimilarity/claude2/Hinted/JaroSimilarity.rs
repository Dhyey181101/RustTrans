

fn jaro_similarity(str1: &str, str2: &str) -> f32 {
    let rune_str1 = str1.chars().collect::<Vec<_>>();
    let rune_str2 = str2.chars().collect::<Vec<_>>();

    let rune_str1_len = rune_str1.len();
    let rune_str2_len = rune_str2.len();
    if rune_str1_len == 0 || rune_str2_len == 0 {
        return 0.0;
    } else if equal(&rune_str1, &rune_str2) {
        return 1.0;
    }

    let mut match_count = 0;
    let max_dist = max(rune_str1_len, rune_str2_len) / 2 - 1;
    let mut str1_table = vec![0; rune_str1_len];
    let mut str2_table = vec![0; rune_str2_len];

    for i in 0..rune_str1_len {
        let min_val = min(rune_str2_len, try_into(i as isize + max_dist as isize + 1).unwrap());
        for j in max(0, (i as isize - max_dist as isize).try_into().unwrap())..min_val {
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

    (match_count as f32 / rune_str1_len as f32 +
     match_count as f32 / rune_str2_len as f32 +
     (match_count as f32 - t) / match_count as f32) / 3.0
}

fn equal(a: &[char], b: &[char]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    a.iter().zip(b).all(|(x, y)| x == y)
}

fn max(a: usize, b: usize) -> usize {
    if b > a {b} else {a}
}

fn min(a: usize, b: usize) -> usize {
    if b < a {b} else {a}
}

fn try_into(value: isize) -> Result<usize, ()> {
    usize::try_from(value).map_err(|_| ())
}


