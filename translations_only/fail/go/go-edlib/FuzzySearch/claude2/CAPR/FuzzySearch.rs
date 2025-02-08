
use std::cmp::{max, min};
use std::collections::HashMap;
use std::error::Error;
use std::f32::consts::SQRT_2;

pub enum Algorithm {
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

pub fn fuzzy_search<'a>(str: &'a str, str_list: &'a [&'a str], algo: &'a Algorithm) -> Result<&'a str, Box<dyn Error+'a>> {
    let mut higher_match_percent = 0.0;
    let mut tmp_str = "";

    for str_to_cmp in str_list {
        let sim = strings_similarity(str, str_to_cmp, algo)?;

        if sim == 1.0 {
            return Ok(str_to_cmp);
        } else if sim > higher_match_percent {
            higher_match_percent = sim;
            tmp_str = *str_to_cmp;
        }
    }

    Ok(tmp_str)
}

pub fn strings_similarity<'a>(str1: &'a str, str2: &'a str, algo: &'a Algorithm) -> Result<f32, Box<dyn Error+'a>> {
    match *algo {
        Algorithm::Levenshtein => Ok(levenshtein_similarity(str1, str2)),
        Algorithm::DamerauLevenshtein => Ok(damerau_levenshtein_similarity(str1, str2)),
        Algorithm::OSADamerauLevenshtein => Ok(osa_damerau_levenshtein_similarity(str1, str2)),
        Algorithm::Lcs => Ok(lcs_similarity(str1, str2)),
        Algorithm::Hamming => {
            let distance = hamming_distance(str1, str2).unwrap_or(0);
            Ok(hamming_similarity(str1, distance))
        }
        Algorithm::Jaro => Ok(jaro_similarity(str1, str2)),
        Algorithm::JaroWinkler => Ok(jaro_winkler_similarity(str1, str2)),
        Algorithm::Cosine => Ok(cosine_similarity(str1, str2, 2)),
        Algorithm::Jaccard => Ok(jaccard_similarity(str1, str2, 2)),
        Algorithm::SorensenDice => Ok(sorensen_dice_coefficient(str1, str2, 2)),
        Algorithm::Qgram => Ok(qgram_similarity(str1, str2, 2)),
    }
}

fn levenshtein_similarity(str1: &str, str2: &str) -> f32 {
    let distance = levenshtein_distance(str1, str2);
    let max_len = max(str1.len(), str2.len());
    if max_len == 0 {
        1.0
    } else {
        1.0 - (distance as f32 / max_len as f32)
    }
}

fn damerau_levenshtein_similarity(str1: &str, str2: &str) -> f32 {
    let distance = damerau_levenshtein_distance(str1, str2);
    let max_len = max(str1.len(), str2.len());
    if max_len == 0 {
        1.0
    } else {
        1.0 - (distance as f32 / max_len as f32)
    }
}

fn osa_damerau_levenshtein_similarity(str1: &str, str2: &str) -> f32 {
    let distance = osa_damerau_levenshtein_distance(str1, str2);
    let max_len = max(str1.len(), str2.len());
    if max_len == 0 {
        1.0
    } else {
        1.0 - (distance as f32 / max_len as f32)
    }  
}

fn lcs_similarity(str1: &str, str2: &str) -> f32 {
    let lcs_len = lcs(str1, str2);
    let max_len = max(str1.len(), str2.len());
    if max_len == 0 {
        1.0
    } else {
        lcs_len as f32 / max_len as f32
    }
}

fn hamming_similarity(_: &str, distance: usize) -> f32 {
    let len = 0;
    if len == 0 {
        1.0
    } else {
        1.0 - (distance as f32 / len as f32)
    }
}

fn levenshtein_distance(_: &str, _: &str) -> usize {
    0
}

fn damerau_levenshtein_distance(_: &str, _: &str) -> usize {
    0
}

fn osa_damerau_levenshtein_distance(_: &str, _: &str) -> usize {
    0  
}

fn lcs(_: &str, _: &str) -> usize {
    0
}

fn hamming_distance(_: &str, _: &str) -> Result<usize, Box<dyn Error>> {
    Ok(0)
}

fn jaro_similarity(_: &str, _: &str) -> f32 {
    0.0
}

fn jaro_winkler_similarity(_: &str, _: &str) -> f32 {
    0.0
}

fn cosine_similarity(_: &str, _: &str, _: usize) -> f32 {
    0.0
}

fn shingle_slice(_: &str, _: usize) -> Vec<String> {
    Vec::new()  
}

fn union(_: &mut Vec<Vec<char>>, _: &mut Vec<Vec<char>>) -> Vec<Vec<char>> {
    Vec::new()
}

fn find(_: &[Vec<char>], _: &[char]) -> isize {
    0
}

fn sum(_: &[i32]) -> i32 {
    0
}

fn jaccard_similarity(_: &str, _: &str, _: usize) -> f32 {
    0.0
}

fn sorensen_dice_coefficient(_: &str, _: &str, _: usize) -> f32 {
    0.0
}  

fn shingle(_: &str, _: usize) -> HashMap<String, i32> {
    HashMap::new()
}

fn qgram_similarity(_: &str, _: &str, _: usize) -> f32 {
    0.0
}

fn qgram_distance_custom_ngram(_: &HashMap<String, i32>, _: &HashMap<String, i32>) -> i32 {
    0
}

