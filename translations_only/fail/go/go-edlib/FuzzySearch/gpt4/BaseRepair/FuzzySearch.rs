
use std::collections::HashMap;
use std::error::Error;
use std::f32;

#[derive(Clone, Copy)]
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
    let len1 = str1.len();
    let len2 = str2.len();
    let mut column: Vec<usize> = (0..=len1).collect();

    for x in 1..=len2 {
        column[0] = x;
        let mut lastdiag = x - 1;
        for y in 1..=len1 {
            let olddiag = column[y];
            let cost = if str1[y - 1] == str2[x - 1] { 0 } else { 1 };
            column[y] = *[
                column[y] + 1,
                column[y - 1] + 1,
                lastdiag + cost,
            ]
            .iter()
            .min()
            .unwrap();
            lastdiag = olddiag;
        }
    }
    column[len1]
}

fn matching_index(str1: &str, str2: &str, distance: isize) -> f32 {
    let len1 = str1.chars().count() as isize;
    let len2 = str2.chars().count() as isize;
    if len1 >= len2 {
        (len1 - distance) as f32 / len1 as f32
    } else {
        (len2 - distance) as f32 / len2 as f32
    }
}

fn damerau_levenshtein_distance(str1: &str, str2: &str) -> usize {
    // Implementation omitted for brevity, similar to levenshtein_distance with transpositions
    0
}

fn osa_damerau_levenshtein_distance(str1: &str, str2: &str) -> usize {
    // Implementation omitted for brevity, similar to damerau_levenshtein_distance with restrictions
    0
}

fn lcs_edit_distance(str1: &str, str2: &str) -> usize {
    // Implementation omitted for brevity, based on the length of the longest common subsequence
    0
}

fn hamming_distance(str1: &str, str2: &str) -> Result<usize, Box<dyn Error>> {
    if str1.len() != str2.len() {
        return Err("Undefined for strings of unequal length".into());
    }
    Ok(str1.chars().zip(str2.chars()).filter(|&(a, b)| a != b).count())
}

fn jaro_similarity(str1: &str, str2: &str) -> f32 {
    // Implementation omitted for brevity, based on the Jaro similarity algorithm
    0.0
}

fn jaro_winkler_similarity(str1: &str, str2: &str) -> f32 {
    // Implementation omitted for brevity, extends the Jaro similarity with prefix scaling
    0.0
}

fn cosine_similarity(str1: &str, str2: &str, split_length: usize) -> f32 {
    // Implementation omitted for brevity, based on the cosine similarity algorithm
    0.0
}

fn jaccard_similarity(str1: &str, str2: &str, split_length: usize) -> f32 {
    // Implementation omitted for brevity, based on the Jaccard similarity algorithm
    0.0
}

fn sorensen_dice_coefficient(str1: &str, str2: &str, split_length: usize) -> f32 {
    // Implementation omitted for brevity, based on the Sørensen–Dice coefficient
    0.0
}

fn qgram_similarity(str1: &str, str2: &str, split_length: usize) -> f32 {
    // Implementation omitted for brevity, based on the q-gram similarity algorithm
    0.0
}
