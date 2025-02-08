
use std::cmp::Ordering;

fn damerau_levenshtein_distance(str1: &str, str2: &str) -> isize {
    let rune_str1: Vec<char> = str1.chars().collect();
    let rune_str2: Vec<char> = str2.chars().collect();

    let rune_str1_len = rune_str1.len();
    let rune_str2_len = rune_str2.len();
    if rune_str1_len == 0 {
        return rune_str2_len as isize;
    } else if rune_str2_len == 0 {
        return rune_str1_len as isize;
    } else if equal(&rune_str1, &rune_str2) {
        return 0;
    }

    let mut da: std::collections::HashMap<char, usize> = std::collections::HashMap::new();
    for i in 0..rune_str1_len {
        da.insert(rune_str1[i], 0);
    }
    for i in 0..rune_str2_len {
        da.insert(rune_str2[i], 0);
    }

    let max_dist = rune_str1_len + rune_str2_len;
    let mut matrix: Vec<Vec<isize>> = vec![vec![0; rune_str2_len + 2]; rune_str1_len + 2];

    matrix[0][0] = max_dist as isize;
    for i in 0..=rune_str1_len {
        matrix[i + 1][0] = max_dist as isize;
        matrix[i + 1][1] = i as isize;
    }
    for i in 0..=rune_str2_len {
        matrix[0][i + 1] = max_dist as isize;
        matrix[1][i + 1] = i as isize;
    }

    for i in 1..=rune_str1_len {
        let mut db = 0;
        for j in 1..=rune_str2_len {
            let i1 = da.get(&rune_str2[j - 1]).unwrap_or(&0);
            let j1 = db;
            let cost = if rune_str1[i - 1] == rune_str2[j - 1] {
                0
            } else {
                1
            };

            matrix[i + 1][j + 1] = min(
                min(
                    matrix[i + 1][j] + 1,
                    matrix[i][j + 1] + 1,
                ),
                min(
                    matrix[i][j] + cost,
                    matrix[(*i1) as usize][j1 as usize] + (i - (*i1) as usize) as isize + 1 + (j - j1) as isize,
                ),
            );
        }

        da.insert(rune_str1[i - 1], i as usize);
    }

    matrix[rune_str1_len + 1][rune_str2_len + 1]
}

fn equal(a: &[char], b: &[char]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    a.iter().zip(b).all(|(x, y)| x == y)
}

fn min(a: isize, b: isize) -> isize {
    match a.cmp(&b) {
        Ordering::Less => a,
        Ordering::Greater | Ordering::Equal => b,
    }
}

