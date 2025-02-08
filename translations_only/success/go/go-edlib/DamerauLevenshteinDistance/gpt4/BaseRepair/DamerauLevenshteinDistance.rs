
fn damerau_levenshtein_distance(str1: &str, str2: &str) -> isize {
    let rune_str1: Vec<char> = str1.chars().collect();
    let rune_str2: Vec<char> = str2.chars().collect();

    let rune_str1len = rune_str1.len();
    let rune_str2len = rune_str2.len();
    if rune_str1len == 0 {
        return rune_str2len as isize;
    } else if rune_str2len == 0 {
        return rune_str1len as isize;
    } else if equal(&rune_str1, &rune_str2) {
        return 0;
    }

    let mut da: std::collections::HashMap<char, isize> = std::collections::HashMap::new();
    for &val in rune_str1.iter().chain(rune_str2.iter()) {
        da.entry(val).or_insert(0);
    }

    let max_dist = (rune_str1len + rune_str2len) as isize;
    let mut matrix = vec![vec![0; rune_str2len + 2]; rune_str1len + 2];

    matrix[0][0] = max_dist;
    for i in 0..=rune_str1len {
        matrix[i + 1][0] = max_dist;
        matrix[i + 1][1] = i as isize;
    }
    for i in 0..=rune_str2len {
        matrix[0][i + 1] = max_dist;
        matrix[1][i + 1] = i as isize;
    }

    let mut cost;
    for i in 1..=rune_str1len {
        let mut db = 0;
        for j in 1..=rune_str2len {
            let i1 = *da.get(&rune_str2[j - 1]).unwrap_or(&0);
            let j1 = db;
            if rune_str1[i - 1] == rune_str2[j - 1] {
                cost = 0;
                db = j as isize;
            } else {
                cost = 1;
            }

            matrix[i + 1][j + 1] = min(
                min(matrix[i + 1][j] + 1, matrix[i][j + 1] + 1),
                min(matrix[i][j] + cost, matrix[i1 as usize][j1 as usize] + (i as isize - i1 - 1) + 1 + (j as isize - j1 - 1)),
            );
        }

        da.insert(rune_str1[i - 1], i as isize);
    }

    matrix[rune_str1len + 1][rune_str2len + 1]
}

fn equal(a: &[char], b: &[char]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    for (i, &v) in a.iter().enumerate() {
        if v != b[i] {
            return false;
        }
    }
    true
}

fn min(a: isize, b: isize) -> isize {
    if b < a { b } else { a }
}
