
fn lcs_edit_distance(str1: &str, str2: &str) -> isize {
    if str1.len() == 0 {
        return str2.len() as isize;
    } else if str2.len() == 0 {
        return str1.len() as isize;
    } else if str1 == str2 {
        return 0;
    }

    let lcs = lcs(str1, str2);
    ((str1.chars().count() as isize) - lcs) + ((str2.chars().count() as isize) - lcs)
}

fn lcs(str1: &str, str2: &str) -> isize {
    let rune_str1: Vec<char> = str1.chars().collect();
    let rune_str2: Vec<char> = str2.chars().collect();

    if rune_str1.len() == 0 || rune_str2.len() == 0 {
        return 0;
    } else if rune_str1 == rune_str2 {
        return rune_str1.len() as isize;
    }

    let lcs_matrix = lcs_process(&rune_str1, &rune_str2);
    lcs_matrix[rune_str1.len()][rune_str2.len()]
}

fn equal(a: &Vec<char>, b: &Vec<char>) -> bool {
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

fn lcs_process(rune_str1: &Vec<char>, rune_str2: &Vec<char>) -> Vec<Vec<isize>> {
    let mut lcs_matrix: Vec<Vec<isize>> = Vec::with_capacity(rune_str1.len() + 1);
    for i in 0..=rune_str1.len() {
        lcs_matrix.push(vec![0; rune_str2.len() + 1]);
    }

    for i in 1..=rune_str1.len() {
        for j in 1..=rune_str2.len() {
            if rune_str1[i - 1] == rune_str2[j - 1] {
                lcs_matrix[i][j] = lcs_matrix[i - 1][j - 1] + 1;
            } else {
                lcs_matrix[i][j] = max(lcs_matrix[i][j - 1], lcs_matrix[i - 1][j]);
            }
        }
    }

    lcs_matrix
}

fn max(a: isize, b: isize) -> isize {
    if b > a {
        b
    } else {
        a
    }
}
