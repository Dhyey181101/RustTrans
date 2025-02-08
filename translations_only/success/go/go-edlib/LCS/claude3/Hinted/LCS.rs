
fn lcs(str1: &str, str2: &str) -> isize {
    let runestr1: Vec<char> = str1.chars().collect();
    let runestr2: Vec<char> = str2.chars().collect();

    if runestr1.is_empty() || runestr2.is_empty() {
        return 0;
    } else if equal(&runestr1, &runestr2) {
        return runestr1.len() as isize;
    }

    let lcs_matrix = lcs_process(&runestr1, &runestr2);
    lcs_matrix[runestr1.len()][runestr2.len()]
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

fn lcs_process(runestr1: &[char], runestr2: &[char]) -> Box<Vec<Vec<isize>>> {
    let mut lcs_matrix: Vec<Vec<isize>> = vec![vec![0; runestr2.len() + 1]; runestr1.len() + 1];

    for i in 1..=runestr1.len() {
        for j in 1..=runestr2.len() {
            if runestr1[i - 1] == runestr2[j - 1] {
                lcs_matrix[i][j] = lcs_matrix[i - 1][j - 1] + 1;
            } else {
                lcs_matrix[i][j] = max(lcs_matrix[i][j - 1], lcs_matrix[i - 1][j]);
            }
        }
    }

    Box::new(lcs_matrix)
}

fn max(a: isize, b: isize) -> isize {
    if b > a {
        b
    } else {
        a
    }
}
