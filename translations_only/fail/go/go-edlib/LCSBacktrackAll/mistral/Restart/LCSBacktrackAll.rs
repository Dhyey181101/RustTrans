

use std::cmp::max;

fn lcs_backtrack_all(str1: &str, str2: &str) -> Result<Vec<String>, &'static str> {
    let rune_str1 = str1.chars().collect::<Vec<char>>();
    let rune_str2 = str2.chars().collect::<Vec<char>>();

    if rune_str1.is_empty() || rune_str2.is_empty() {
        return Err("Can't process and backtrack any LCS with empty string");
    } else if are_equal(&rune_str1, &rune_str2) {
        return Ok(vec![str1.to_string()]);
    }

    let lcs_matrix = lcs_process(&rune_str1, &rune_str2);

    Ok(process_lcs_backtrack_all(
        str1,
        str2,
        &lcs_matrix,
        rune_str1.len(),
        rune_str2.len(),
    ))
}

fn are_equal(a: &[char], b: &[char]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    a.iter().zip(b).all(|(&x, &y)| x == y)
}

fn lcs_process(rune_str1: &[char], rune_str2: &[char]) -> Vec<Vec<usize>> {
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

    lcs_matrix
}

struct StringHashMap {
    map: std::collections::HashMap<String, ()>,
}

impl StringHashMap {
    fn new() -> Self {
        Self {
            map: std::collections::HashMap::new(),
        }
    }

    fn add(&mut self, key: String) {
        self.map.insert(key, ());
    }

    fn add_all(&mut self, src_map: &StringHashMap) {
        for (key, _) in src_map.map.iter() {
            self.add(key.clone());
        }
    }

    fn to_array(self) -> Vec<String> {
        self.map.keys().cloned().collect()
    }
}

fn process_lcs_backtrack_all(
    str1: &str,
    str2: &str,
    lcs_matrix: &[Vec<usize>],
    m: usize,
    n: usize,
) -> Vec<String> {
    let rune_str1 = str1.chars().collect::<Vec<char>>();
    let rune_str2 = str2.chars().collect::<Vec<char>>();

    let mut substrings = Vec::new();

    if m == 0 || n == 0 {
        substrings.push("".to_string());
    } else if rune_str1[m - 1] == rune_str2[n - 1] {
        let mut substrings_ref =
            process_lcs_backtrack_all(str1, str2, lcs_matrix, m - 1, n - 1);
        for s in substrings_ref {
            substrings.push(format!("{}{}", s, rune_str1[m - 1]));
        }
    } else {
        if lcs_matrix[m - 1][n] >= lcs_matrix[m][n - 1] {
            let mut substrings_ref =
                process_lcs_backtrack_all(str1, str2, lcs_matrix, m - 1, n);
            for s in substrings_ref {
                substrings.push(s);
            }
        }
        if lcs_matrix[m][n - 1] >= lcs_matrix[m - 1][n] {
            let mut substrings_ref =
                process_lcs_backtrack_all(str1, str2, lcs_matrix, m, n - 1);
            for s in substrings_ref {
                substrings.push(s);
            }
        }
    }

    substrings
}

fn main() {
    let result = lcs_backtrack_all("hello", "world");

    match result {
        Ok(v) => println!("{:?}", v),
        Err(e) => println!("{}", e),
    }
}

