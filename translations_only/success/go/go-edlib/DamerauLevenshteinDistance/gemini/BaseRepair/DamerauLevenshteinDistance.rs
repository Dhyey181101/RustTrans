
use std::cmp::min;
use std::collections::HashMap;

pub fn damerau_levenshtein_distance(str1: &str, str2: &str) -> usize {
    let rune_str1: Vec<char> = str1.chars().collect();
    let rune_str2: Vec<char> = str2.chars().collect();

    let rune_str1_len = rune_str1.len();
    let rune_str2_len = rune_str2.len();

    if rune_str1_len == 0 {
        return rune_str2_len;
    } else if rune_str2_len == 0 {
        return rune_str1_len;
    } else if rune_str1 == rune_str2 {
        return 0;
    }

    let mut da: HashMap<char, usize> = HashMap::new();
    for i in 0..rune_str1_len {
        da.insert(rune_str1[i], 0);
    }
    for i in 0..rune_str2_len {
        da.insert(rune_str2[i], 0);
    }

    let mut matrix: Vec<Vec<usize>> = vec![vec![0; rune_str2_len + 2]; rune_str1_len + 2];

    let max_dist = rune_str1_len + rune_str2_len;

    matrix[0][0] = max_dist;
    for i in 0..rune_str1_len + 1 {
        matrix[i + 1][0] = max_dist;
        matrix[i + 1][1] = i;
    }
    for i in 0..rune_str2_len + 1 {
        matrix[0][i + 1] = max_dist;
        matrix[1][i + 1] = i;
    }

    let mut cost: usize;
    for i in 1..rune_str1_len + 1 {
        let mut db = 0;
        for j in 1..rune_str2_len + 1 {
            let i1 = *da.get(&rune_str2[j - 1]).unwrap();
            let j1 = db;
            if rune_str1[i - 1] == rune_str2[j - 1] {
                cost = 0;
                db = j;
            } else {
                cost = 1;
            }

            matrix[i + 1][j + 1] = min(
                min(matrix[i + 1][j] + 1, matrix[i][j + 1] + 1),
                min(matrix[i][j] + cost, matrix[i1][j1] + (i - i1 - 1) + 1 + (j - j1 - 1)),
            );
        }

        da.insert(rune_str1[i - 1], i);
    }

    matrix[rune_str1_len + 1][rune_str2_len + 1]
}
