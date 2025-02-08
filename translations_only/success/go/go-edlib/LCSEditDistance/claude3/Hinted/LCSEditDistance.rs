
fn lcs_edit_distance(str1: &str, str2: &str) -> isize {
    if str1.is_empty() {
        return str2.len() as isize;
    } else if str2.is_empty() {
        return str1.len() as isize;
    } else if str1 == str2 {
        return 0;
    }

    let lcs = lcs(str1, str2);
    return (str1.chars().count() - lcs) as isize + (str2.chars().count() - lcs) as isize;
}

fn lcs(str1: &str, str2: &str) -> usize {
    if str1.is_empty() || str2.is_empty() {
        return 0;
    } else if str1 == str2 {
        return str1.len();
    }

    let lcs_matrix = lcs_process(str1.chars().collect(), str2.chars().collect());
    return lcs_matrix[str1.len()][str2.len()];
}

fn lcs_process(str1: Vec<char>, str2: Vec<char>) -> Box<Vec<Vec<usize>>> {
    let mut lcs_matrix = vec![vec![0; str2.len() + 1]; str1.len() + 1];

    for i in 1..=str1.len() {
        for j in 1..=str2.len() {
            if str1[i - 1] == str2[j - 1] {
                lcs_matrix[i][j] = lcs_matrix[i - 1][j - 1] + 1;
            } else {
                lcs_matrix[i][j] = max(lcs_matrix[i][j - 1], lcs_matrix[i - 1][j]);
            }
        }
    }

    Box::new(lcs_matrix)
}

fn max(a: usize, b: usize) -> usize {
    if b > a {
        b
    } else {
        a
    }
}
