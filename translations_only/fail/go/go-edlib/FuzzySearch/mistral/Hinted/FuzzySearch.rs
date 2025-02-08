


use std::cmp::min;
use std::collections::HashMap;
use std::iter;

const LEVENSHTEIN: u8 = 0;
const DAMERAU_LEVENSHTEIN: u8 = 1;
const OSADAMERAU_LEVENSHTEIN: u8 = 2;
const LCS: u8 = 3;
const HAMMING: u8 = 4;
const JARO: u8 = 5;
const JARO_WINKLER: u8 = 6;
const COSINE: u8 = 7;
const JACCARD: u8 = 8;
const SORENSEN_DICE: u8 = 9;
const QGRAM: u8 = 10;

fn fuzzy_search(str: &str, str_list: &[&str], algo: u8) -> (String, Option<String>) {
    let mut higher_match_percent = 0.0;
    let mut tmp_str = String::new();
    for str_to_cmp in str_list {
        let (sim, err) = strings_similarity(str.to_string(), str_to_cmp.to_string(), algo);
        if err.is_some() {
            return (String::new(), err);
        }

        if sim == 1.0 {
            return (str_to_cmp.to_string(), None);
        } else if sim > higher_match_percent {
            higher_match_percent = sim;
            tmp_str = str_to_cmp.to_string();
        }
    }

    (tmp_str, None)
}

fn strings_similarity(
    str1: String,
    str2: String,
    algo: u8,
) -> (f32, Option<String>) {
    match algo {
        LEVENSHTEIN => (matching_index(str1.clone(), str2.clone(), levenshtein_distance(str1, str2)), None),
        DAMERAU_LEVENSHTEIN => (
            matching_index(str1.clone(), str2.clone(), damerau_levenshtein_distance(str1, str2)),
            None,
        ),
        OSADAMERAU_LEVENSHTEIN => (
            matching_index(str1.clone(), str2.clone(), osa_damerau_levenshtein_distance(str1, str2)),
            None,
        ),
        LCS => (matching_index(str1.clone(), str2.clone(), lcs_edit_distance(str1, str2)), None),
        HAMMING => {
            if str1.len() != str2.len() {
                return (0.0, Some("Undefined for strings of unequal length".to_string()));
            }
            (
                matching_index(str1.clone(), str2.clone(), hamming_distance(str1, str2)),
                None,
            )
        }
        _ => (0.0, None),
    }
}

// Implement the Levenshtein distance algorithm
fn levenshtein_distance(str1: String, str2: String) -> u32 {
    let m = str1.len();
    let n = str2.len();

    if m == 0 {
        return n as u32;
    }
    if n == 0 {
        return m as u32;
    }

    let mut matrix = vec![vec![0; n + 1]; m + 1];

    for i in 0..=m {
        matrix[i][0] = i as u32;
    }
    for j in 0..=n {
        matrix[0][j] = j as u32;
    }

    for i in 1..=m {
        for j in 1..=n {
            let cost = if str1.chars().nth(i - 1).unwrap()
                == str2.chars().nth(j - 1).unwrap()
            {
                0
            } else {
                1
            };

            matrix[i][j] = min(matrix[i - 1][j] + 1, matrix[i][j - 1] + 1);
            matrix[i][j] = min(matrix[i][j], matrix[i - 1][j - 1] + cost);
        }
    }

    matrix[m][n]
}

// Implement the Damerau-Levenshtein distance algorithm
fn damerau_levenshtein_distance(str1: String, str2: String) -> u32 {
    let m = str1.len();
    let n = str2.len();

    if m == 0 {
        return n as u32;
    }
    if n == 0 {
        return m as u32;
    }

    let mut matrix = vec![vec![0; n + 1]; m + 1];

    for i in 0..=m {
        matrix[i][0] = i as u32;
    }
    for j in 0..=n {
        matrix[0][j] = j as u32;
    }

    for i in 1..=m {
        for j in 1..=n {
            let cost = if str1.chars().nth(i - 1).unwrap()
                == str2.chars().nth(j - 1).unwrap()
            {
                0
            } else {
                1
            };

            matrix[i][j] = min(matrix[i - 1][j] + 1, matrix[i][j - 1] + 1);
            matrix[i][j] = min(matrix[i][j], matrix[i - 1][j - 1] + cost);

            if i > 1 && j > 1 && str1.chars().nth(i - 1).unwrap()
                == str2.chars().nth(j - 2).unwrap()
                && str1.chars().nth(i - 2).unwrap()
                    == str2.chars().nth(j - 1).unwrap()
            {
                matrix[i][j] =
                    min(matrix[i][j], matrix[i - 2][j - 2] + cost);
            }
        }
    }

    matrix[m][n]
}

// Implement the Optimal String Alignment (OSA) Damerau-Levenshtein distance algorithm
fn osa_damerau_levenshtein_distance(str1: String, str2: String) -> u32 {
    let m = str1.len();
    let n = str2.len();

    if m == 0 {
        return n as u32;
    }
    if n == 0 {
        return m as u32;
    }

    let mut matrix = vec![vec![0; n + 2]; m + 2];

    for i in 0..=m + 1 {
        matrix[i][0] = i as u32;
    }
    for j in 0..=n + 1 {
        matrix[0][j] = j as u32;
    }

    for i in 1..=m {
        for j in 1..=n {
            let cost = if str1.chars().nth(i - 1).unwrap()
                == str2.chars().nth(j - 1).unwrap()
            {
                0
            } else {
                1
            };

            matrix[i + 1][j + 1] = min(
                matrix[i][j + 1] + 1,
                min(matrix[i + 1][j] + 1, matrix[i][j] + cost),
            );

            if i > 1 && j > 1 && str1.chars().nth(i - 1).unwrap()
                == str2.chars().nth(j - 2).unwrap()
                && str1.chars().nth(i - 2).unwrap()
                    == str2.chars().nth(j - 1).unwrap()
            {
                matrix[i + 1][j + 1] =
                    min(matrix[i + 1][j + 1], matrix[i - 1][j - 1] + cost);
            }
        }
    }

    matrix[m + 1][n + 1] - 1
}

// Implement the Longest Common Subsequence (LCS) edit distance algorithm
fn lcs_edit_distance(str1: String, str2: String) -> u32 {
    let m = str1.len();
    let n = str2.len();

    if m == 0 {
        return n as u32;
    }
    if n == 0 {
        return m as u32;
    }

    let mut matrix = vec![vec![0; n + 1]; m + 1];

    for i in 0..=m {
        matrix[i][0] = i as u32;
    }
    for j in 0..=n {
        matrix[0][j] = j as u32;
    }

    for i in 1..=m {
        for j in 1..=n {
            if str1.chars().nth(i - 1).unwrap() == str2.chars().nth(j - 1).unwrap() {
                matrix[i][j] = matrix[i - 1][j - 1];
            } else {
                matrix[i][j] = 1 + min(matrix[i - 1][j], matrix[i][j - 1]);
            }
        }
    }

    matrix[m][n]
}

// Implement the Hamming distance algorithm
fn hamming_distance(str1: String, str2: String) -> u32 {
    if str1.len() != str2.len() {
        return 0;
    }

    let mut dist = 0;
    for i in 0..str1.len() {
        if str1.chars().nth(i).unwrap() != str2.chars().nth(i).unwrap() {
            dist += 1;
        }
    }

    dist
}

// Implement the matching index function
fn matching_index(str1: String, str2: String, distance: u32) -> f32 {
    let m = str1.len();
    let n = str2.len();

    if m == 0 || n == 0 {
        return 0.0;
    }

    let max_len = min(m, n);

    let mut num_matches = 0;
    for i in 0..max_len {
        if str1.chars().nth(i).unwrap() == str2.chars().nth(i).unwrap() {
            num_matches += 1;
        }
    }

    (2.0 * num_matches as f32) / (m as f32 + n as f32) - (distance as f32 / m as f32)
}


