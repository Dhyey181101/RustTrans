
fn lcseditdistance(str1: &str, str2: &str) -> isize {
    if str1.len() == 0 {
        return str2.len() as isize;
    } else if str2.len() == 0 {
        return str1.len() as isize;
    } else if str1 == str2 {
        return 0;
    }

    let lcs = lcs(str1, str2);
    (str1.len() as isize - lcs as isize) + (str2.len() as isize - lcs as isize)
}

fn lcs(str1: &str, str2: &str) -> isize {
    let runestr1 = str1.as_bytes();
    let runestr2 = str2.as_bytes();

    if runestr1.len() == 0 || runestr2.len() == 0 {
        return 0;
    } else if equal(runestr1, runestr2) {
        return runestr1.len() as isize;
    }

    let lcs_matrix = lcs_process(runestr1, runestr2);
    lcs_matrix[runestr1.len()][runestr2.len()] as isize
}

fn equal(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    a.iter().zip(b).all(|(a, b)| a == b)
}

fn lcs_process(runestr1: &[u8], runestr2: &[u8]) -> Vec<Vec<isize>> {
    let mut lcs_matrix = vec![vec![0; runestr2.len() + 1]; runestr1.len() + 1];

    for i in 1..=runestr1.len() {
        for j in 1..=runestr2.len() {
            if runestr1[i-1] == runestr2[j-1] {
                lcs_matrix[i][j] = lcs_matrix[i-1][j-1] + 1;
            } else {
                lcs_matrix[i][j] = max(lcs_matrix[i][j-1], lcs_matrix[i-1][j]);
            }
        }
    }

    lcs_matrix
}

fn max(a: isize, b: isize) -> isize {
    if b > a {b} else {a}
}

