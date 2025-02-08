
use std::cmp::max;
use std::cmp::min;

const MAX: isize = i32::MAX as isize;
const MIN: isize = i32::MIN as isize;

fn jaro_similarity(str1: &str, str2: &str) -> f32 {
    let rune_str1 = str1.chars().collect::<Vec<char>>();
    let rune_str2 = str2.chars().collect::<Vec<char>>();

    let rune_str1len = rune_str1.len() as isize;
    let rune_str2len = rune_str2.len() as isize;
    if rune_str1len == 0 || rune_str2len == 0 {
        0.0
    } else if are_equal(&rune_str1, &rune_str2) {
        1.0
    } else {
        let mut match_count = 0;
        let max_dist = max(rune_str1len, rune_str2len) / 2 - 1;
        let mut str1_table = vec![0; rune_str1len as usize];
        let mut str2_table = vec![0; rune_str2len as usize];

        for i in 0..rune_str1len {
            for j in max(0, i as isize - max_dist)..min(rune_str2len, i as isize + max_dist + 1) {
                if rune_str1[i as usize] == rune_str2[j as usize] && str2_table[j as usize] == 0 {
                    str1_table[i as usize] = 1;
                    str2_table[j as usize] = 1;
                    match_count += 1;
                    break;
                }
            }
        }

        if match_count == 0 {
            0.0
        } else {
            let mut t = 0.0;
            let mut p = 0;
            for (i, v) in str1_table.into_iter().enumerate() {
                if v == 1 {
                    while str2_table[p] == 0 {
                        p += 1;
                    }
                    if rune_str1[i] != rune_str2[p] {
                        t += 0.5;
                    }
                    p += 1;
                }
            }
            (match_count as f32 / rune_str1len as f32
                + match_count as f32 / rune_str2len as f32
                + (match_count as f32 - t) / match_count as f32) / 3.0
        }
    }
}

fn are_equal(a: &Vec<char>, b: &Vec<char>) -> bool {
    if a.len() != b.len() {
        false
    } else {
        for i in 0..a.len() {
            if a[i] != b[i] {
                return false;
            }
        }
        true
    }
}
