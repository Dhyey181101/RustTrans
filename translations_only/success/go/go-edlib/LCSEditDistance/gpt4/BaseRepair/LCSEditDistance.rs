
fn lcse_distance(str1: &str, str2: &str) -> isize {
    if str1.is_empty() {
        return str2.chars().count() as isize;
    } else if str2.is_empty() {
        return str1.chars().count() as isize;
    } else if str1 == str2 {
        return 0;
    }

    let lcs = lcs(str1, str2);
    (str1.chars().count() as isize - lcs) + (str2.chars().count() as isize - lcs)
}

fn lcs(str1: &str, str2: &str) -> isize {
    let rune_str1: Vec<char> = str1.chars().collect();
    let rune_str2: Vec<char> = str2.chars().collect();

    if rune_str1.is_empty() || rune_str2.is_empty() {
        return 0;
    } else if rune_str1 == rune_str2 {
        return rune_str1.len() as isize;
    }

    let lcs_matrix = lcs_process(&rune_str1, &rune_str2);
    lcs_matrix[rune_str1.len()][rune_str2.len()]
}

fn lcs_process(rune_str1: &[char], rune_str2: &[char]) -> Box<Vec<Vec<isize>>> {
    let mut lcs_matrix = vec![vec![0; rune_str2.len() + 1]; rune_str1.len() + 1];

    for i in 1..=rune_str1.len() {
        for j in 1..=rune_str2.len() {
            if rune_str1[i - 1] == rune_str2[j - 1] {
                lcs_matrix[i][j] = lcs_matrix[i - 1][j - 1] + 1;
            } else {
                lcs_matrix[i][j] = max(lcs_matrix[i][j - 1], lcs_matrix[i - 1][j]);
            }
        }
    }

    Box::new(lcs_matrix)
}

fn max(a: isize, b: isize) -> isize {
    if b > a { b } else { a }
}
