

use std::collections::HashMap;

fn DamerauLevenshteinDistance(str1: String, str2: String) -> isize {
    let rune_str1 = str1.chars().collect::<Vec<char>>();
    let rune_str2 = str2.chars().collect::<Vec<char>>();

    let rune_str1len = rune_str1.len();
    let rune_str2len = rune_str2.len();

    if rune_str1len == 0 {
        return rune_str2len as isize;
    } else if rune_str2len == 0 {
        return rune_str1len as isize;
    } else if Equal(&rune_str1, &rune_str2) {
        return 0;
    }

    let mut da: HashMap<char, usize> = HashMap::new();
    for i in 0..rune_str1len {
        da.insert(rune_str1[i], i);
    }
    for i in 0..rune_str2len {
        da.insert(rune_str2[i], i);
    }

    let mut matrix = vec![vec![0; rune_str2len + 2]; rune_str1len + 2];

    let max_dist = (rune_str1len + rune_str2len) as isize;

    matrix[0][0] = max_dist;
    for i in 0..=rune_str1len {
        matrix[i+1][0] = max_dist;
        matrix[i+1][1] = i as isize;
    }
    for i in 0..=rune_str2len {
        matrix[0][i+1] = max_dist;
        matrix[1][i+1] = i as isize;
    }

    let mut db = 0;
    for i in 1..=rune_str1len {
        for j in 1..=rune_str2len {
            let i1 = *da.get(&rune_str2[j-1]).unwrap_or(&usize::MAX);
            let j1 = db;
            let cost = if rune_str1[i-1] == rune_str2[j-1] {
                0
            } else {
                1
            };

            matrix[i+1][j+1] = isize::min(
                isize::min(
                    matrix[i+1][j]+1,
                    matrix[i][j+1]+1
                ),
                isize::min(
                    matrix[i][j]+cost,
                    matrix[i1][j1]+(i as isize-i1 as isize-1)+1+(j as isize-j1 as isize-1)
                )
            );
            db = j;
            da.insert(rune_str1[i-1], i);
        }
    }

    matrix[rune_str1len+1][rune_str2len+1] as isize
}

fn Equal(a: &Vec<char>, b: &Vec<char>) -> bool {
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

fn min(a: isize, b: isize) -> isize {
    if b < a {
        return b;
    }
    return a;
}

