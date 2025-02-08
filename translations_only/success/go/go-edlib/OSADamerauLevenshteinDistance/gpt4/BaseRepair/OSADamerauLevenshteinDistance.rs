
fn osa_damerau_levenshtein_distance(str1: &str, str2: &str) -> isize {
    let rune_str1: Vec<char> = str1.chars().collect();
    let rune_str2: Vec<char> = str2.chars().collect();

    let rune_str1len = rune_str1.len() as isize;
    let rune_str2len = rune_str2.len() as isize;
    if rune_str1len == 0 {
        return rune_str2len;
    } else if rune_str2len == 0 {
        return rune_str1len;
    } else if equal(&rune_str1, &rune_str2) {
        return 0;
    } else if rune_str1len < rune_str2len {
        return osa_damerau_levenshtein_distance(str2, str1);
    }

    let row = min(rune_str1len + 1, 3);
    let mut matrix = vec![vec![0; rune_str2len as usize + 1]; row as usize];
    for i in 0..row {
        matrix[i as usize][0] = i;
    }

    for j in 0..=rune_str2len {
        matrix[0][j as usize] = j;
    }

    let mut count;
    for i in 1..=rune_str1len {
        matrix[(i % 3) as usize][0] = i;
        for j in 1..=rune_str2len {
            if rune_str1[(i - 1) as usize] == rune_str2[(j - 1) as usize] {
                count = 0;
            } else {
                count = 1;
            }

            matrix[(i % 3) as usize][j as usize] = min(
                min(
                    matrix[((i - 1) % 3) as usize][j as usize] + 1,
                    matrix[(i % 3) as usize][(j - 1) as usize] + 1,
                ),
                matrix[((i - 1) % 3) as usize][(j - 1) as usize] + count,
            );
            if i > 1 && j > 1
                && rune_str1[(i - 1) as usize] == rune_str2[(j - 2) as usize]
                && rune_str1[(i - 2) as usize] == rune_str2[(j - 1) as usize]
            {
                matrix[(i % 3) as usize][j as usize] = min(
                    matrix[(i % 3) as usize][j as usize],
                    matrix[((i - 2) % 3) as usize][(j - 2) as usize] + 1,
                );
            }
        }
    }
    matrix[(rune_str1len % 3) as usize][rune_str2len as usize]
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
    if b < a {
        b
    } else {
        a
    }
}
