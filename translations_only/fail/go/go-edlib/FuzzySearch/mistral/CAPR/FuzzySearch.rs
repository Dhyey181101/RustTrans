




use std::cmp::min;
use std::collections::HashMap;
use std::iter;

const LEVENSHTEIN: u8 = 0;
const DAMERAU_LEVENSHTEIN: u8 = 1;
const OS_ADAMERAU_LEVENSHTEIN: u8 = 2;
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
        let (sim, err) = strings_similarity(str, str_to_cmp, algo);
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

fn strings_similarity(str1: &str, str2: &str, algo: u8) -> (f32, Option<String>) {
    match algo {
        LEVENSHTEIN => (matching_index(str1, str2, levenshtein_distance(str1, str2) as f32), None),
        DAMERAU_LEVENSHTEIN => (
            matching_index(str1, str2, damerau_levenshtein_distance(str1, str2) as f32),
            None,
        ),
        OS_ADAMERAU_LEVENSHTEIN => (
            matching_index(str1, str2, osa_damerau_levenshtein_distance(str1, str2) as f32),
            None,
        ),
        LCS => (matching_index(str1, str2, lcs_edit_distance(str1, str2) as f32), None),
        HAMMING => {
            match hamming_distance(str1, str2) {
                Ok(val) => (val as f32 / str1.len() as f32, None),
                Err(_) => (0.0, Some(String::from("Error in input string"))),
            }
        }
        _ => (0.0, Some(String::from("Invalid algorithm"))),
    }
}

fn levenshtein_distance(str1: &str, str2: &str) -> u32 {
    let m = str1.len();
    let n = str2.len();

    let mut matrix = vec![vec![0; n + 1]; m + 1];

    for i in 1..=m {
        matrix[i][0] = i as u32;
    }
    for j in 1..=n {
        matrix[0][j] = j as u32;
    }

    for i in 1..=m {
        for j in 1..=n {
            let cost = if str1.chars().nth(i - 1).unwrap() == str2.chars().nth(j - 1).unwrap() {
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

fn damerau_levenshtein_distance(str1: &str, str2: &str) -> u32 {
    let m = str1.len();
    let n = str2.len();

    let mut matrix = vec![vec![0; n + 1]; m + 1];

    for i in 1..=m {
        matrix[i][0] = i as u32;
    }
    for j in 1..=n {
        matrix[0][j] = j as u32;
    }

    for i in 1..=m {
        for j in 1..=n {
            let cost = if str1.chars().nth(i - 1).unwrap() == str2.chars().nth(j - 1).unwrap() {
                0
            } else {
                1
            };

            matrix[i][j] = min(matrix[i - 1][j] + 1, matrix[i][j - 1] + 1);
            if i > 1 && j > 1 && str1.chars().nth(i - 1).unwrap() == str2.chars().nth(j - 2).unwrap()
                && str1.chars().nth(i - 2).unwrap() == str2.chars().nth(j - 1).unwrap()
            {
                matrix[i][j] = min(matrix[i][j], matrix[i - 2][j - 2] + cost);
            }
        }
    }

    matrix[m][n]
}

fn osa_damerau_levenshtein_distance(str1: &str, str2: &str) -> u32 {
    let m = str1.len();
    let n = str2.len();

    let mut matrix = vec![vec![0; n + 1]; m + 1];

    for i in 1..=m {
        matrix[i][0] = i as u32;
    }
    for j in 1..=n {
        matrix[0][j] = j as u32;
    }

    for i in 1..=m {
        for j in 1..=n {
            let cost = if str1.chars().nth(i - 1).unwrap() == str2.chars().nth(j - 1).unwrap() {
                0
            } else {
                1
            };

            matrix[i][j] = min(matrix[i - 1][j] + 1, matrix[i][j - 1] + 1);
            if i > 1 && j > 1 && str1.chars().nth(i - 1).unwrap() == str2.chars().nth(j - 2).unwrap() {
                matrix[i][j] =
                    min(matrix[i][j], matrix[i - 1][j - 1] + cost);
            }
            if i > 1 && j > 0 && str1.chars().nth(i - 1).unwrap() == str2.chars().nth(j).unwrap() {
                matrix[i][j] =
                    min(matrix[i][j], matrix[i - 2][j] + cost);
            }
        }
    }

    matrix[m][n]
}

fn lcs_edit_distance(str1: &str, str2: &str) -> u32 {
    let m = str1.len();
    let n = str2.len();

    let mut matrix = vec![vec![0; n + 1]; m + 1];

    for i in 1..=m {
        for j in 1..=n {
            if str1.chars().nth(i - 1).unwrap() == str2.chars().nth(j - 1).unwrap() {
                matrix[i][j] = matrix[i - 1][j - 1] + 1;
            } else {
                matrix[i][j] = std::cmp::max(matrix[i - 1][j], matrix[i][j - 1]);
            }
        }
    }

    (str1.len() as i32 + str2.len() as i32 - 2 * matrix[m][n] as i32) as u32
}

fn hamming_distance(str1: &str, str2: &str) -> Result<u32, ()> {
    if str1.len() != str2.len() {
        return Err(());
    }

    let mut dist = 0;
    for (c1, c2) in str1.chars().zip(str2.chars()) {
        if c1 != c2 {
            dist += 1;
        }
    }

    Ok(dist)
}

fn matching_index(str1: &str, str2: &str, threshold: f32) -> f32 {
    let len = min(str1.len(), str2.len());
    let mut matches = 0;

    for i in 0..len {
        if str1.chars().nth(i).unwrap() == str2.chars().nth(i).unwrap() {
            matches += 1;
        }
    }

    matches as f32 / len as f32
}




