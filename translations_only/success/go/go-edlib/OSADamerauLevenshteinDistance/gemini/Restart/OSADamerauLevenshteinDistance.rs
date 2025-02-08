
fn osadamerau_levenshtein_distance(str1: &str, str2: &str) -> isize {
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
    } else if rune_str1len < rune_str2len {
        return osadamerau_levenshtein_distance(str2, str1);
    }

    let row = std::cmp::min(rune_str1len + 1, 3);
    let mut matrix: Vec<Vec<isize>> = Vec::with_capacity(row);
    for i in 0..row {
        matrix.push(vec![i as isize; rune_str2len + 1]);
    }

    for j in 0..=rune_str2len {
        matrix[0][j] = j as isize;
    }

    let mut count = 0;
    for i in 1..=rune_str1len {
        matrix[i % 3][0] = i as isize;
        for j in 1..=rune_str2len {
            if rune_str1[i - 1] == rune_str2[j - 1] {
                count = 0;
            } else {
                count = 1;
            }

            matrix[i % 3][j] = std::cmp::min(
                std::cmp::min(matrix[(i - 1) % 3][j] + 1, matrix[i % 3][j - 1] + 1),
                matrix[(i - 1) % 3][j - 1] + count,
            );
            if i > 1 && j > 1 && rune_str1[i - 1] == rune_str2[j - 2] && rune_str1[i - 2] == rune_str2[j - 1] {
                matrix[i % 3][j] = std::cmp::min(matrix[i % 3][j], matrix[(i - 2) % 3][j - 2] + 1);
            }
        }
    }
    return matrix[rune_str1len % 3][rune_str2len];
}
