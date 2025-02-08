
use std::cmp::max;

struct StringHashMap {
    data: std::collections::HashMap<String, ()>,
}

impl StringHashMap {
    fn new() -> Self {
        Self {
            data: std::collections::HashMap::new(),
        }
    }

    fn add(&mut self, key: String) {
        self.data.insert(key, ());
    }

    fn add_all(&mut self, src_map: Self) {
        for (key, _) in src_map.data {
            self.data.insert(key, ());
        }
    }

    fn to_array(&self) -> Vec<String> {
        self.data.keys().cloned().collect()
    }
}

fn lcs_process(rune_str1: &[u32], rune_str2: &[u32]) -> Vec<Vec<usize>> {
    let m = rune_str1.len();
    let n = rune_str2.len();

    let mut lcs_matrix = vec![vec![0; n + 1]; m + 1];

    for i in 1..=m {
        for j in 1..=n {
            if rune_str1[i - 1] == rune_str2[j - 1] {
                lcs_matrix[i][j] = lcs_matrix[i - 1][j - 1] + 1;
            } else {
                lcs_matrix[i][j] = max(lcs_matrix[i][j - 1], lcs_matrix[i - 1][j]);
            }
        }
    }

    lcs_matrix
}

fn process_lcs_backtrack_all(
    str1: &str,
    str2: &str,
    lcs_matrix: &[Vec<usize>],
    m: usize,
    n: usize,
) -> StringHashMap {
    let mut substrings = StringHashMap::new();

    if m == 0 || n == 0 {
        substrings.add("".to_string());
    } else if str1.chars().nth(m - 1).unwrap() == str2.chars().nth(n - 1).unwrap() {
        for key in process_lcs_backtrack_all(str1, str2, lcs_matrix, m - 1, n - 1).to_array() {
            substrings.add(key + &str1[m - 1..m]);
        }
    } else {
        if lcs_matrix[m - 1][n] >= lcs_matrix[m][n - 1] {
            substrings.add_all(process_lcs_backtrack_all(str1, str2, lcs_matrix, m - 1, n));
        }
        if lcs_matrix[m][n - 1] >= lcs_matrix[m - 1][n] {
            substrings.add_all(process_lcs_backtrack_all(str1, str2, lcs_matrix, m, n - 1));
        }
    }

    substrings
}

fn equal(a: &[u32], b: &[u32]) -> bool {
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

fn lcs_backtrack_all(str1: &str, str2: &str) -> (Vec<String>, Option<String>) {
    let rune_str1: Vec<u32> = str1.chars().map(|c| c as u32).collect();
    let rune_str2: Vec<u32> = str2.chars().map(|c| c as u32).collect();

    if rune_str1.is_empty() || rune_str2.is_empty() {
        return (Vec::new(), Some("Can't process and backtrack any LCS with empty string".to_string()));
    } else if equal(&rune_str1, &rune_str2) {
        return (vec![str1.to_string()], None);
    }

    let lcs_matrix = lcs_process(&rune_str1, &rune_str2);

    let substrings = process_lcs_backtrack_all(str1, str2, &lcs_matrix, rune_str1.len(), rune_str2.len());

    (substrings.to_array(), None)
}

fn main() {}
