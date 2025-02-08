

use std::cmp;

fn jaro_similarity(str1: &str, str2: &str) -> f32 {
    let runestr1: Vec<char> = str1.chars().collect();
    let runestr2: Vec<char> = str2.chars().collect();

    let runestr1len = runestr1.len();
    let runestr2len = runestr2.len();

    if runestr1len == 0 || runestr2len == 0 {
        return 0.0;
    } else if equal(&runestr1, &runestr2) {
        return 1.0;
    }

    let mut match_count = 0;
    let max_dist = cmp::max(runestr1len, runestr2len) / 2;
    let mut str1table = vec![0; runestr1len];
    let mut str2table = vec![0; runestr2len];

    for i in 0..runestr1len {
        for j in cmp::max(0, i as isize - max_dist as isize) as usize
            ..cmp::min(runestr2len as isize, i as isize + max_dist as isize + 1) as usize
        {
            if runestr1[i] == runestr2[j] && str2table[j] == 0 {
                str1table[i] = 1;
                str2table[j] = 1;
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
    for i in 0..runestr1len {
        if str1table[i] == 1 {
            while str2table[p] == 0 {
                p += 1;
            }
            if runestr1[i] != runestr2[p] {
                t += 1.0;
            }
            p += 1;
        }
    }
    t /= 2.0;

    (match_count as f32 / runestr1len as f32
        + match_count as f32 / runestr2len as f32
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


