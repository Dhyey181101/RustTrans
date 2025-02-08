
use std::collections::HashMap;

fn damerau_levenshtein_distance(str1: &str, str2: &str) -> isize {
    // Convert string parameters to char vectors to be compatible with non-ASCII
    let mut rune_str1: Vec<char> = str1.chars().collect();
    let mut rune_str2: Vec<char> = str2.chars().collect();

    // Get and store length of these strings
    let rune_str1len = rune_str1.len();
    let rune_str2len = rune_str2.len();
    if rune_str1len == 0 {
        return rune_str2len as isize;
    } else if rune_str2len == 0 {
        return rune_str1len as isize;
    } else if equal(&rune_str1, &rune_str2) {
        return 0;
    }

    // Create alphabet based on input strings
    let mut da = HashMap::new();
    for i in 0..rune_str1len {
        da.insert(rune_str1[i], 0);
    }
    for i in 0..rune_str2len {
        da.insert(rune_str2[i], 0);
    }

    // 2D Vector for distance matrix : matrix[0..str1.length+2][0..s2.length+2]
    let mut matrix: Vec<Vec<isize>> = vec![vec![0; rune_str2len + 2]; rune_str1len + 2];

    // Maximum possible distance
    let max_dist = (rune_str1len + rune_str2len) as isize;

    // Initialize matrix
    matrix[0][0] = max_dist;
    for i in 0..=rune_str1len {
        matrix[i + 1][0] = max_dist;
        matrix[i + 1][1] = i as isize;
    }
    for i in 0..=rune_str2len {
        matrix[0][i + 1] = max_dist;
        matrix[1][i + 1] = i as isize;
    }

    // Process edit distance
    for i in 1..=rune_str1len {
        let mut db = 0;
        for j in 1..=rune_str2len {
            let i1 = *da.get(&rune_str2[j - 1]).unwrap_or(&0);
            let j1 = db;
            let cost = if rune_str1[i - 1] == rune_str2[j - 1] {
                db = j;
                0
            } else {
                1
            };

            matrix[i + 1][j + 1] = min(
                min(
                    matrix[i + 1][j] + 1,  // Addition
                    matrix[i][j + 1] + 1), // Deletion
                min(
                    matrix[i][j] + cost, // Substitution
                    matrix[i1][j1] + (i - i1 - 1) as isize + 1 + (j - j1 - 1) as isize), // Transposition
            );
        }

        da.insert(rune_str1[i - 1], i);
    }

    matrix[rune_str1len + 1][rune_str2len + 1]
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

fn min(a: isize, b: isize) -> isize {
    if b < a {
        b
    } else {
        a
    }
}
