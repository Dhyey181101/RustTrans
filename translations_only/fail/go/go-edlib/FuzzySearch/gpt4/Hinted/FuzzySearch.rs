
use std::collections::HashMap;
use std::error::Error;
use std::f32;

#[derive(Debug, Clone, Copy)]
enum Algorithm {
    Levenshtein,
    DamerauLevenshtein,
    OSADamerauLevenshtein,
    Lcs,
    Hamming,
    Jaro,
    JaroWinkler,
    Cosine,
    Jaccard,
    SorensenDice,
    Qgram,
}

fn fuzzy_search(str: &str, str_list: Vec<&str>, algo: Algorithm) -> Result<String, Box<dyn Error>> {
    let mut higher_match_percent: f32 = 0.0;
    let mut tmp_str = String::new();
    for str_to_cmp in str_list {
        let sim = strings_similarity(str, str_to_cmp, algo)?;
        if sim == 1.0 {
            return Ok(str_to_cmp.to_string());
        } else if sim > higher_match_percent {
            higher_match_percent = sim;
            tmp_str = str_to_cmp.to_string();
        }
    }
    Ok(tmp_str)
}

fn strings_similarity(str1: &str, str2: &str, algo: Algorithm) -> Result<f32, Box<dyn Error>> {
    match algo {
        Algorithm::Levenshtein => Ok(matching_index(str1, str2, levenshtein_distance(str1, str2) as isize)),
        Algorithm::DamerauLevenshtein => Ok(matching_index(str1, str2, damerau_levenshtein_distance(str1, str2) as isize)),
        Algorithm::OSADamerauLevenshtein => Ok(matching_index(str1, str2, osa_damerau_levenshtein_distance(str1, str2) as isize)),
        Algorithm::Lcs => Ok(matching_index(str1, str2, lcs_edit_distance(str1, str2) as isize)),
        Algorithm::Hamming => {
            let distance = hamming_distance(str1, str2)?;
            Ok(matching_index(str1, str2, distance as isize))
        },
        Algorithm::Jaro => Ok(jaro_similarity(str1, str2)),
        Algorithm::JaroWinkler => Ok(jaro_winkler_similarity(str1, str2)),
        Algorithm::Cosine => Ok(cosine_similarity(str1, str2, 2)),
        Algorithm::Jaccard => Ok(jaccard_similarity(str1, str2, 2)),
        Algorithm::SorensenDice => Ok(sorensen_dice_coefficient(str1, str2, 2)),
        Algorithm::Qgram => Ok(qgram_similarity(str1, str2, 2)),
    }
}

fn levenshtein_distance(str1: &str, str2: &str) -> usize {
    let str1: Vec<char> = str1.chars().collect();
    let str2: Vec<char> = str2.chars().collect();
    let str1_len = str1.len();
    let str2_len = str2.len();
    if str1_len == 0 {
        return str2_len;
    }
    if str2_len == 0 {
        return str1_len;
    }
    if str1 == str2 {
        return 0;
    }

    let mut column: Vec<usize> = (0..=str1_len).collect();

    for x in 1..=str2_len {
        column[0] = x;
        let mut lastkey = x - 1;
        for y in 1..=str1_len {
            let oldkey = column[y];
            let mut i = 1;
            if str1[y - 1] == str2[x - 1] {
                i = 0;
            }
            let val = std::cmp::min(std::cmp::min(column[y] + 1, column[y - 1] + 1), lastkey + i);
            column[y] = val;
            lastkey = oldkey;
        }
    }

    column[str1_len]
}

fn matching_index(str1: &str, str2: &str, distance: isize) -> f32 {
    let str1_len = str1.chars().count() as isize;
    let str2_len = str2.chars().count() as isize;
    if str1_len >= str2_len {
        (str1_len - distance) as f32 / str1_len as f32
    } else {
        (str2_len - distance) as f32 / str2_len as f32
    }
}

fn damerau_levenshtein_distance(str1: &str, str2: &str) -> usize {
    // Placeholder for the actual implementation
    0
}

fn osa_damerau_levenshtein_distance(str1: &str, str2: &str) -> usize {
    // Placeholder for the actual implementation
    0
}

fn lcs_edit_distance(str1: &str, str2: &str) -> usize {
    // Placeholder for the actual implementation
    0
}

fn hamming_distance(str1: &str, str2: &str) -> Result<usize, Box<dyn Error>> {
    if str1.len() != str2.len() {
        return Err("Undefined for strings of unequal length".into());
    }
    Ok(str1.chars().zip(str2.chars()).filter(|&(a, b)| a != b).count())
}

fn jaro_similarity(str1: &str, str2: &str) -> f32 {
    // Placeholder for the actual implementation
    0.0
}

fn jaro_winkler_similarity(str1: &str, str2: &str) -> f32 {
    // Placeholder for the actual implementation
    0.0
}

fn cosine_similarity(str1: &str, str2: &str, split_length: usize) -> f32 {
    // Placeholder for the actual implementation
    0.0
}

fn jaccard_similarity(str1: &str, str2: &str, split_length: usize) -> f32 {
    // Placeholder for the actual implementation
    0.0
}

fn sorensen_dice_coefficient(str1: &str, str2: &str, split_length: usize) -> f32 {
    // Placeholder for the actual implementation
    0.0
}

fn qgram_similarity(str1: &str, str2: &str, split_length: usize) -> f32 {
    // Placeholder for the actual implementation
    0.0
}
