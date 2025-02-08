
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
    let str1_chars: Vec<char> = str1.chars().collect();
    let str2_chars: Vec<char> = str2.chars().collect();

    if str1_chars.is_empty() || str2_chars.is_empty() {
        return 0;
    } else if str1_chars == str2_chars {
        return str1_chars.len();
    }

    let lcs_matrix = lcs_process(&str1_chars, &str2_chars);
    return lcs_matrix[str1_chars.len()][str2_chars.len()];
}

fn lcs_process(str1_chars: &Vec<char>, str2_chars: &Vec<char>) -> Box<Vec<Vec<usize>>> {
    let mut lcs_matrix: Box<Vec<Vec<usize>>> = Box::new(vec![vec![0; str2_chars.len() + 1]; str1_chars.len() + 1]);

    for i in 1..=str1_chars.len() {
        for j in 1..=str2_chars.len() {
            if str1_chars[i - 1] == str2_chars[j - 1] {
                lcs_matrix[i][j] = lcs_matrix[i - 1][j - 1] + 1;
            } else {
                lcs_matrix[i][j] = max(lcs_matrix[i][j - 1], lcs_matrix[i - 1][j]);
            }
        }
    }

    lcs_matrix
}

fn max(a: usize, b: usize) -> usize {
    if b > a {
        b
    } else {
        a
    }
}
