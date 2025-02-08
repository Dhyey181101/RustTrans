
fn osa_damerau_levenshtein_distance(str1: &str, str2: &str) -> isize {
    let rune_str1 = str1.chars().collect::<Vec<_>>();
    let rune_str2 = str2.chars().collect::<Vec<_>>();

    let rune_str1len = rune_str1.len();
    let rune_str2len = rune_str2.len();
    if rune_str1len == 0 {
        return rune_str2len as isize;
    } else if rune_str2len == 0 {
        return rune_str1len as isize;
    } else if equal(rune_str1.as_slice(), rune_str2.as_slice()) {
        return 0;
    } else if rune_str1len < rune_str2len {
        return osa_damerau_levenshtein_distance(str2, str1);
    }

    let mut matrix = vec![vec![0; rune_str2len + 1]; 3];
    for i in 0..3 {
        matrix[i][0] = i as isize;
    }
    for j in 0..=rune_str2len {
        matrix[0][j] = j as isize;
    }

    let mut count;
    for i in 1..=rune_str1len {
        matrix[(i % 3) as usize][0] = i as isize;
        for j in 1..=rune_str2len {
            if rune_str1[i - 1] == rune_str2[j - 1] {
                count = 0;
            } else {
                count = 1;
            }

            matrix[(i % 3) as usize][j] = min(
                min(
                    matrix[((i - 1) % 3) as usize][j] + 1,
                    matrix[(i % 3) as usize][j - 1] + 1,
                ),
                matrix[((i - 1) % 3) as usize][j - 1] + count,
            );
            if i > 1
                && j > 1
                && rune_str1[i - 1] == rune_str2[j - 2]
                && rune_str1[i - 2] == rune_str2[j - 1]
            {
                matrix[(i % 3) as usize][j] = min(matrix[(i % 3) as usize][j], matrix[((i - 2) % 3) as usize][j - 2] + 1);
            }
        }
    }
    matrix[(rune_str1len % 3) as usize][rune_str2len]
}

fn equal(a: &[char], b: &[char]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    a.iter().zip(b).all(|(a, b)| a == b)
}

fn min(a: isize, b: isize) -> isize {
    if b < a {
        b
    } else {
        a
    }
}
