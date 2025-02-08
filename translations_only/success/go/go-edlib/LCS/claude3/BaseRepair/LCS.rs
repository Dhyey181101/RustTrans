
fn lcs(str1: &str, str2: &str) -> isize {
    let runestr1: Vec<char> = str1.chars().collect();
    let runestr2: Vec<char> = str2.chars().collect();

    if runestr1.is_empty() || runestr2.is_empty() {
        return 0;
    } else if equal(&runestr1, &runestr2) {
        return runestr1.len() as isize;
    }

    let lcsmatrix = lcs_process(&runestr1, &runestr2);
    return lcsmatrix[runestr1.len()][runestr2.len()];
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

fn lcs_process(runestr1: &Vec<char>, runestr2: &Vec<char>) -> Box<Vec<Vec<isize>>> {
    let mut lcsmatrix: Box<Vec<Vec<isize>>> = Box::new(vec![vec![0; runestr2.len() + 1]; runestr1.len() + 1]);

    for i in 1..=runestr1.len() {
        for j in 1..=runestr2.len() {
            if runestr1[i - 1] == runestr2[j - 1] {
                lcsmatrix[i][j] = lcsmatrix[i - 1][j - 1] + 1;
            } else {
                lcsmatrix[i][j] = max(lcsmatrix[i][j - 1], lcsmatrix[i - 1][j]);
            }
        }
    }

    lcsmatrix
}

fn max(a: isize, b: isize) -> isize {
    if b > a {
        b
    } else {
        a
    }
}
