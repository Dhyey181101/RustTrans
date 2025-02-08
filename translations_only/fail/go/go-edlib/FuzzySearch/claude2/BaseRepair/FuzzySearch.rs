

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

pub fn fuzzy_search<'a>(str: &'a str, str_list: &'a [&'a str], algo: &'a Algorithm) -> Result<&'a str, Box<dyn Error>> {
    let mut higher_match_percent = 0.0;
    let mut tmp_str = "";

    for str_to_cmp in str_list {
        let sim = strings_similarity(str, str_to_cmp, algo)?;

        if sim == 1.0 {
            return Ok(str_to_cmp);
        } else if sim > higher_match_percent {
            higher_match_percent = sim;
            tmp_str = str_to_cmp;
        }
    }

    Ok(tmp_str)
}

pub fn strings_similarity<'a>(str1: &'a str, str2: &'a str, algo: &'a Algorithm) -> Result<f32, Box<dyn Error>> {
    match *algo {
        Algorithm::Levenshtein => Ok(matching_index(str1, str2, levenshtein_distance(str1, str2))),
        Algorithm::DamerauLevenshtein => Ok(matching_index(str1, str2, damerau_levenshtein_distance(str1, str2))),
        Algorithm::OSADamerauLevenshtein => Ok(matching_index(str1, str2, osa_damerau_levenshtein_distance(str1, str2))),
        Algorithm::Lcs => Ok(matching_index(str1, str2, lcs_edit_distance(str1, str2))),
        Algorithm::Hamming => {
            let distance = hamming_distance(str1, str2)?;
            Ok(matching_index(str1, str2, distance))
        }
        Algorithm::Jaro => Ok(jaro_similarity(str1, str2)),
        Algorithm::JaroWinkler => Ok(jaro_winkler_similarity(str1, str2)),
        Algorithm::Cosine => Ok(cosine_similarity(str1, str2, 2)),
        Algorithm::Jaccard => Ok(jaccard_similarity(str1, str2, 2)),
        Algorithm::SorensenDice => Ok(sorensen_dice_coefficient(str1, str2, 2)),
        Algorithm::Qgram => Ok(qgram_similarity(str1, str2, 2)),
        _ => return Err("Illegal argument for algorithm method".into())
    }
}

fn levenshtein_distance(str1: &str, str2: &str) -> isize {
    0
}

fn matching_index(str1: &str, str2: &str, distance: isize) -> f32 {
    0.0  
}

fn damerau_levenshtein_distance(str1: &str, str2: &str) -> isize {
    0
}

fn osa_damerau_levenshtein_distance(str1: &str, str2: &str) -> isize {
    0
}

fn lcs_edit_distance(str1: &str, str2: &str) -> isize {
    0
}

fn lcs(str1: &str, str2: &str) -> isize {
    0
}

fn hamming_distance(_: &str, _: &str) -> Result<isize, Box<dyn Error>> {
    Ok(0)
}

fn jaro_similarity(str1: &str, str2: &str) -> f32 {
    0.0
}

fn jaro_winkler_similarity(str1: &str, str2: &str) -> f32 {
    0.0
}

fn cosine_similarity(str1: &str, str2: &str, split_length: usize) -> f32 {
    0.0
}

fn shingle_slice(s: &str, k: usize) -> Vec<String> {
    Vec::new()
}

fn union(a: &[String], b: &[String]) -> Vec<Vec<u8>> {
    Vec::new()  
}

fn find(slice: &[Vec<u8>], val: &[u8]) -> isize {
    0
}

fn sum(arr: Vec<isize>) -> isize {
    0
}

fn jaccard_similarity(str1: &str, str2: &str, split_length: usize) -> f32 {
    0.0
}

fn sorensen_dice_coefficient(str1: &str, str2: &str, split_length: usize) -> f32 {
    0.0
}

fn shingle(s: &str, k: usize) -> HashMap<String, i32> {
    HashMap::new()
}

fn qgram_similarity(str1: &str, str2: &str, split_length: usize) -> f32 {
    0.0  
}


