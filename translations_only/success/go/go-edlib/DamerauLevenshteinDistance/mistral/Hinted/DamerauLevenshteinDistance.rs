

use std::collections::HashMap;
use std::cmp::min;

fn DamerauLevenshteinDistance(str1: &str, str2: &str) -> isize {
    let rune_str1: Vec<char> = str1.chars().collect();
    let rune_str2: Vec<char> = str2.chars().collect();

    let rune_str1len = rune_str1.len();
    let rune_str2len = rune_str2.len();
    if rune_str1len == 0 {
        return rune_str2len as isize;
    } else if rune_str2len == 0 {
        return rune_str1len as isize;
    } else if rune_str1 == rune_str2 {
        return 0;
    }

    let mut da: HashMap<char, isize> = HashMap::new();
    for i in 0..rune_str1len {
        da.insert(rune_str1[i], 0);
    }
    for i in 0..rune_str2len {
        da.insert(rune_str2[i], 0);
    }

    let mut matrix: Vec<Vec<isize>> = vec![vec![0; rune_str2len + 2]; rune_str1len + 2];

    let max_dist = rune_str1len as isize + rune_str2len as isize;

    matrix[0][0] = max_dist;
    for i in 0..rune_str1len + 1 {
        matrix[i+1][0] = max_dist;
        matrix[i+1][1] = i as isize;
    }
    for i in 0..rune_str2len + 1 {
        matrix[0][i+1] = max_dist;
        matrix[1][i+1] = i as isize;
    }

    let mut db = 0;
    for i in 1..rune_str1len + 1 {
        let i1 = *da.get(&rune_str2[i-1]).unwrap_or(&0);
        for j in 1..rune_str2len + 1 {
            let cost = if rune_str1[i-1] == rune_str2[j-1] {
                0
            } else {
                1
            };

            matrix[i+1][j+1] = min(
                min(
                    matrix[i+1][j] + 1,  // Addition
                    matrix[i][j+1] + 1), // Deletion
                min(
                    matrix[i][j] + cost, // Substitution
                    matrix[(i as usize).saturating_sub(i1 as usize)][db as usize] + (i as isize - i1 as isize - 1) + 1 + (j as isize - db as isize - 1))); // Transposition
            db = db + 1;
        }

        da.insert(rune_str1[i-1], i as isize);
    }

    matrix[rune_str1len+1][rune_str2len+1]
}

fn Equal(a: &[char], b: &[char]) -> bool {
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

fn Min(a: isize, b: isize) -> isize {
    if b < a {
        return b;
    }
    return a;
}

