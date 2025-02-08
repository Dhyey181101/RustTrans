
use std::cmp::min as std_min;

fn damerau_levenshtein_distance(str1: &str, str2: &str) -> isize {
    let rune_str1 = str1.chars().collect::<Vec<_>>();
    let rune_str2 = str2.chars().collect::<Vec<_>>();

    let rune_str1len = rune_str1.len();
    let rune_str2len = rune_str2.len();
    if rune_str1len == 0 {
        return rune_str2len as isize;
    } else if rune_str2len == 0 {
        return rune_str1len as isize;
    } else if equal(&rune_str1, &rune_str2) {
        return 0;
    }

    let mut da = std::collections::HashMap::new();
    for i in 0..rune_str1len {
        da.insert(rune_str1[i], 0);
    }
    for i in 0..rune_str2len {
        da.insert(rune_str2[i], 0);
    }

    let mut matrix = vec![vec![0; rune_str2len + 2]; rune_str1len + 2];

    let max_dist = rune_str1len + rune_str2len;

    matrix[0][0] = max_dist as i32;
    for i in 0..=rune_str1len {
        matrix[i + 1][0] = max_dist as i32;
        matrix[i + 1][1] = i as i32;
    }
    for i in 0..=rune_str2len {
        matrix[0][i + 1] = max_dist as i32;
        matrix[1][i + 1] = i as i32;
    }

    let mut cost;
    for i in 1..=rune_str1len {
        let mut db = 0;
        for j in 1..=rune_str2len {
            let i1 = *da.get(&rune_str2[j - 1]).unwrap_or(&0);
            let j1 = db;
            if rune_str1[i - 1] == rune_str2[j - 1] {
                cost = 0;
                db = j;
            } else {
                cost = 1;
            }

            matrix[i + 1][j + 1] = min(
                min(
                    matrix[i + 1][j] + 1,
                    matrix[i][j + 1] + 1
                ),
                min(
                    matrix[i][j] + cost,
                    matrix[i1 as usize][j1 as usize] + ((i - i1) - 1) as i32 + 1 + ((j - j1) - 1) as i32
                )
            );
        }

        da.insert(rune_str1[i - 1], i);
    }

    matrix[rune_str1len + 1][rune_str2len + 1] as isize
}

fn equal(a: &[char], b: &[char]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    a.iter().zip(b).all(|(x, y)| x == y)
}

fn min(a: i32, b: i32) -> i32 {
    if b < a {
        b
    } else {
        a
    }
}

