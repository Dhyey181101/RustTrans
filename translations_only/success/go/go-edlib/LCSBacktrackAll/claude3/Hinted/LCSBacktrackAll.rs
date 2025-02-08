
use std::collections::HashMap;

fn lcs_backtrack_all(str1: &str, str2: &str) -> Result<Vec<String>, &'static str> {
    let runestr1: Vec<char> = str1.chars().collect();
    let runestr2: Vec<char> = str2.chars().collect();

    if runestr1.is_empty() || runestr2.is_empty() {
        return Err("Can't process and backtrack any LCS with empty string");
    } else if equal(&runestr1, &runestr2) {
        return Ok(vec![str1.to_string()]);
    }

    Ok(process_lcs_backtrack_all(
        str1,
        str2,
        lcs_process(&runestr1, &runestr2),
        runestr1.len(),
        runestr2.len(),
    )
    .into_iter()
    .collect())
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

fn lcs_process(runestr1: &[char], runestr2: &[char]) -> Vec<Vec<isize>> {
    let mut lcs_matrix = vec![vec![0; runestr2.len() + 1]; runestr1.len() + 1];

    for i in 1..=runestr1.len() {
        for j in 1..=runestr2.len() {
            if runestr1[i - 1] == runestr2[j - 1] {
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

fn process_lcs_backtrack_all(
    str1: &str,
    str2: &str,
    lcs_matrix: Vec<Vec<isize>>,
    m: usize,
    n: usize,
) -> Vec<String> {
    let runestr1: Vec<char> = str1.chars().collect();
    let runestr2: Vec<char> = str2.chars().collect();

    let mut substrings: HashMap<String, ()> = HashMap::new();

    if m == 0 || n == 0 {
        substrings.insert(String::new(), ());
    } else if runestr1[m - 1] == runestr2[n - 1] {
        for key in process_lcs_backtrack_all(
            str1,
            str2,
            lcs_matrix.clone(),
            m - 1,
            n - 1,
        ) {
            substrings.insert(key + &runestr1[m - 1].to_string(), ());
        }
    } else {
        if lcs_matrix[m - 1][n] >= lcs_matrix[m][n - 1] {
            add_all(
                &mut substrings,
                &process_lcs_backtrack_all(str1, str2, lcs_matrix.clone(), m - 1, n),
            );
        }
        if lcs_matrix[m][n - 1] >= lcs_matrix[m - 1][n] {
            add_all(
                &mut substrings,
                &process_lcs_backtrack_all(str1, str2, lcs_matrix.clone(), m, n - 1),
            );
        }
    }

    substrings.into_keys().collect()
}

fn add_all(map: &mut HashMap<String, ()>, src_map: &[String]) {
    for key in src_map {
        map.insert(key.clone(), ());
    }
}
