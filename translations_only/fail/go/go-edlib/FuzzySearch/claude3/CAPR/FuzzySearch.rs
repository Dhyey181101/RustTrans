

use std::cmp::{max, min};
use std::collections::HashMap;
use std::f32;

#[derive(PartialEq, Eq, Clone, Copy)]
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

pub fn fuzzy_search(str: &str, str_list: &[String], algo: Algorithm) -> (String, Option<()>) {
    let mut higher_match_percent: f32 = 0.0;
    let mut tmp_str = String::new();
    for str_to_cmp in str_list {
        let sim = match strings_similarity(str, str_to_cmp, algo) {
            Ok(sim) => sim,
            Err(_) => return (String::new(), None),
        };

        if sim == 1.0 {
            return (str_to_cmp.clone(), Some(()));
        } else if sim > higher_match_percent {
            higher_match_percent = sim;
            tmp_str = str_to_cmp.clone();
        }
    }

    (tmp_str, Some(()))
}

pub fn strings_similarity(str1: &str, str2: &str, algo: Algorithm) -> Result<f32, ()> {
    match algo {
        Algorithm::Levenshtein => Ok(matching_index(str1, str2, levenshtein_distance(str1, str2))),
        Algorithm::DamerauLevenshtein => {
            Ok(matching_index(str1, str2, damerau_levenshtein_distance(str1, str2)))
        }
        Algorithm::OSADamerauLevenshtein => {
            Ok(matching_index(str1, str2, osa_damerau_levenshtein_distance(str1, str2)))
        }
        Algorithm::Lcs => Ok(matching_index(str1, str2, lcs_edit_distance(str1, str2))),
        Algorithm::Hamming => match hamming_distance(str1, str2) {
            Ok(distance) => Ok(matching_index(str1, str2, distance)),
            Err(_) => Err(()),
        },
        Algorithm::Jaro => Ok(jaro_similarity(str1, str2)),
        Algorithm::JaroWinkler => Ok(jaro_winkler_similarity(str1, str2)),
        Algorithm::Cosine => Ok(cosine_similarity(str1, str2, 2)),
        Algorithm::Jaccard => Ok(jaccard_similarity(str1, str2, 2)),
        Algorithm::SorensenDice => Ok(sorensen_dice_coefficient(str1, str2, 2)),
        Algorithm::Qgram => Ok(qgram_similarity(str1, str2, 2)),
    }
}

pub fn levenshtein_distance(str1: &str, str2: &str) -> isize {
    let rune_str1: Vec<char> = str1.chars().collect();
    let rune_str2: Vec<char> = str2.chars().collect();

    let rune_str1_len = rune_str1.len();
    let rune_str2_len = rune_str2.len();
    if rune_str1_len == 0 {
        return rune_str2_len as isize;
    } else if rune_str2_len == 0 {
        return rune_str1_len as isize;
    } else if equal(&rune_str1, &rune_str2) {
        return 0;
    }

    let mut column = vec![0; rune_str1_len + 1];

    for y in 1..=rune_str1_len {
        column[y] = y as isize;
    }
    for x in 1..=rune_str2_len {
        column[0] = x as isize;
        let mut lastkey = (x - 1) as isize;
        for y in 1..=rune_str1_len {
            let oldkey = column[y];
            let mut i = 0;
            if rune_str1[y - 1] != rune_str2[x - 1] {
                i = 1;
            }
            column[y] = min(
                min(column[y] + 1, column[y - 1] + 1),
                lastkey + i,
            );
            lastkey = oldkey;
        }
    }

    column[rune_str1_len]
}

pub fn equal(a: &[char], b: &[char]) -> bool {
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

pub fn matching_index(str1: &str, str2: &str, distance: isize) -> f32 {
    let rune_str1: Vec<char> = str1.chars().collect();
    let rune_str2: Vec<char> = str2.chars().collect();
    if rune_str1.len() >= rune_str2.len() {
        (rune_str1.len() - distance as usize) as f32 / rune_str1.len() as f32
    } else {
        (rune_str2.len() - distance as usize) as f32 / rune_str2.len() as f32
    }
}

pub fn damerau_levenshtein_distance(str1: &str, str2: &str) -> isize {
    let rune_str1: Vec<char> = str1.chars().collect();
    let rune_str2: Vec<char> = str2.chars().collect();

    let rune_str1_len = rune_str1.len();
    let rune_str2_len = rune_str2.len();
    if rune_str1_len == 0 {
        return rune_str2_len as isize;
    } else if rune_str2_len == 0 {
        return rune_str1_len as isize;
    } else if equal(&rune_str1, &rune_str2) {
        return 0;
    }

    let mut da = HashMap::new();
    for i in 0..rune_str1_len {
        da.insert(rune_str1[i], 0);
    }
    for i in 0..rune_str2_len {
        da.insert(rune_str2[i], 0);
    }

    let max_dist = (rune_str1_len + rune_str2_len) as isize;

    let mut matrix = vec![vec![0; rune_str2_len + 2]; rune_str1_len + 2];

    matrix[0][0] = max_dist;
    for i in 0..=rune_str1_len {
        matrix[i + 1][0] = max_dist;
        matrix[i + 1][1] = i as isize;
    }
    for i in 0..=rune_str2_len {
        matrix[0][i + 1] = max_dist;
        matrix[1][i + 1] = i as isize;
    }

    for i in 1..=rune_str1_len {
        let mut db = 0;
        for j in 1..=rune_str2_len {
            let i1 = *da.get(&rune_str2[j - 1]).unwrap_or(&0);
            let j1 = db;
            let cost = if rune_str1[i - 1] == rune_str2[j - 1] {
                0
            } else {
                1
            };
            db = j;

            matrix[i + 1][j + 1] = min(
                min(matrix[i + 1][j] + 1, matrix[i][j + 1] + 1),
                min(
                    matrix[i][j] + cost,
                    matrix[i1][j1] + (i - i1 - 1) as isize + 1 + (j - j1 - 1) as isize,
                ),
            );
        }

        *da.get_mut(&rune_str1[i - 1]).unwrap() = i;
    }

    matrix[rune_str1_len + 1][rune_str2_len + 1]
}

pub fn osa_damerau_levenshtein_distance(str1: &str, str2: &str) -> isize {
    let rune_str1: Vec<char> = str1.chars().collect();
    let rune_str2: Vec<char> = str2.chars().collect();

    let rune_str1_len = rune_str1.len();
    let rune_str2_len = rune_str2.len();
    if rune_str1_len == 0 {
        return rune_str2_len as isize;
    } else if rune_str2_len == 0 {
        return rune_str1_len as isize;
    } else if equal(&rune_str1, &rune_str2) {
        return 0;
    } else if rune_str1_len < rune_str2_len {
        return osa_damerau_levenshtein_distance(str2, str1);
    }

    let row = min(rune_str1_len + 1, 3);
    let mut matrix = vec![vec![0; rune_str2_len + 1]; row];

    for i in 0..row {
        matrix[i][0] = i as isize;
    }

    for j in 0..=rune_str2_len {
        matrix[0][j] = j as isize;
    }

    for i in 1..=rune_str1_len {
        matrix[i % 3][0] = i as isize;
        for j in 1..=rune_str2_len {
            let count = if rune_str1[i - 1] == rune_str2[j - 1] {
                0
            } else {
                1
            };

            matrix[i % 3][j] = min(
                min(matrix[(i - 1) % 3][j] + 1, matrix[i % 3][j - 1] + 1),
                matrix[(i - 1) % 3][j - 1] + count,
            );
            if i > 1 && j > 1 && rune_str1[i - 1] == rune_str2[j - 2] && rune_str1[i - 2] == rune_str2[j - 1] {
                matrix[i % 3][j] = min(matrix[i % 3][j], matrix[(i - 2) % 3][j - 2] + 1);
            }
        }
    }
    matrix[rune_str1_len % 3][rune_str2_len]
}

pub fn lcs_edit_distance(str1: &str, str2: &str) -> isize {
    if str1.is_empty() {
        return str2.len() as isize;
    } else if str2.is_empty() {
        return str1.len() as isize;
    } else if str1 == str2 {
        return 0;
    }

    let lcs = lcs(str1, str2);
    (str1.len() - lcs) as isize + (str2.len() - lcs) as isize
}

pub fn lcs(str1: &str, str2: &str) -> usize {
    let rune_str1: Vec<char> = str1.chars().collect();
    let rune_str2: Vec<char> = str2.chars().collect();

    if rune_str1.is_empty() || rune_str2.is_empty() {
        return 0;
    } else if equal(&rune_str1, &rune_str2) {
        return rune_str1.len();
    }

    let lcs_matrix = lcs_process(&rune_str1, &rune_str2);
    lcs_matrix[rune_str1.len()][rune_str2.len()]
}

pub fn lcs_process(rune_str1: &[char], rune_str2: &[char]) -> Vec<Vec<usize>> {
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

pub fn hamming_distance(str1: &str, str2: &str) -> Result<isize, ()> {
    let rune_str1: Vec<char> = str1.chars().collect();
    let rune_str2: Vec<char> = str2.chars().collect();

    if rune_str1.len() != rune_str2.len() {
        return Err(());
    } else if equal(&rune_str1, &rune_str2) {
        return Ok(0);
    }

    let mut counter = 0;
    for i in 0..rune_str1.len() {
        if rune_str1[i] != rune_str2[i] {
            counter += 1;
        }
    }

    Ok(counter as isize)
}

pub fn jaro_similarity(str1: &str, str2: &str) -> f32 {
    let rune_str1: Vec<char> = str1.chars().collect();
    let rune_str2: Vec<char> = str2.chars().collect();

    let rune_str1_len = rune_str1.len();
    let rune_str2_len = rune_str2.len();
    if rune_str1_len == 0 || rune_str2_len == 0 {
        return 0.0;
    } else if equal(&rune_str1, &rune_str2) {
        return 1.0;
    }

    let mut match_count = 0;
    let max_dist = max(rune_str1_len, rune_str2_len) / 2 - 1;
    let mut str1_table = vec![0; rune_str1_len];
    let mut str2_table = vec![0; rune_str2_len];

    for i in 0..rune_str1_len {
        for j in max(0, i as isize - max_dist as isize) as usize..min(rune_str2_len, i + max_dist + 1) {
            if rune_str1[i] == rune_str2[j] && str2_table[j] == 0 {
                str1_table[i] = 1;
                str2_table[j] = 1;
                match_count += 1;
                break;
            }
        }
    }
    if match_count == 0 {
        return 0.0;
    }

    let mut t = 0.0;
    let mut p = 0;
    for i in 0..rune_str1_len {
        if str1_table[i] == 1 {
            while str2_table[p] == 0 {
                p += 1;
            }
            if rune_str1[i] != rune_str2[p] {
                t += 1.0;
            }
            p += 1;
        }
    }
    t /= 2.0;

    (match_count as f32 / rune_str1_len as f32
        + match_count as f32 / rune_str2_len as f32
        + (match_count as f32 - t) / match_count as f32)
        / 3.0
}

pub fn jaro_winkler_similarity(str1: &str, str2: &str) -> f32 {
    let jaro_sim = jaro_similarity(str1, str2);

    if jaro_sim != 0.0 && jaro_sim != 1.0 {
        let rune_str1: Vec<char> = str1.chars().collect();
        let rune_str2: Vec<char> = str2.chars().collect();

        let rune_str1_len = rune_str1.len();
        let rune_str2_len = rune_str2.len();

        let mut prefix = 0;

        for i in 0..min(rune_str1_len, rune_str2_len) {
            if rune_str1[i] == rune_str2[i] {
                prefix += 1;
            } else {
                break;
            }
        }

        prefix = min(prefix, 4);

        return jaro_sim + 0.1 * prefix as f32 * (1.0 - jaro_sim);
    }

    jaro_sim
}

pub fn cosine_similarity(str1: &str, str2: &str, split_length: usize) -> f32 {
    if str1.is_empty() || str2.is_empty() {
        return 0.0;
    }

    let splitted_str1: Vec<String> = if split_length == 0 {
        str1.split_whitespace().map(|s| s.to_string()).collect()
    } else {
        shingle_slice(str1, split_length)
    };
    let splitted_str2: Vec<String> = if split_length == 0 {
        str2.split_whitespace().map(|s| s.to_string()).collect()
    } else {
        shingle_slice(str2, split_length)
    };

    let rune_str1: Vec<Vec<char>> = splitted_str1.iter().map(|s| s.chars().collect()).collect();
    let rune_str2: Vec<Vec<char>> = splitted_str2.iter().map(|s| s.chars().collect()).collect();

    let mut l1 = Vec::new();
    let mut l2 = Vec::new();
    let union_str: Vec<Vec<char>> = union(&rune_str1, &rune_str2);
    for word in &union_str {
        let fw = find(&rune_str1, word);
        if fw != -1 {
            l1.push(1);
        } else {
            l1.push(0);
        }

        let fw = find(&rune_str2, word);
        if fw != -1 {
            l2.push(1);
        } else {
            l2.push(0);
        }
    }

    let mut cosine_sim = 0.0;
    for i in 0..union_str.len() {
        cosine_sim += l1[i] as f32 * l2[i] as f32;
    }

    cosine_sim / (sum(&l1) as f32 * sum(&l2) as f32).sqrt()
}

pub fn shingle_slice(s: &str, k: usize) -> Vec<String> {
    let mut out = Vec::new();
    let mut m = HashMap::new();
    if !s.is_empty() && k != 0 {
        let rune_s: Vec<char> = s.chars().collect();
        for i in 0..rune_s.len() - k + 1 {
            let key: String = rune_s[i..i + k].iter().collect();
            *m.entry(key).or_insert(0) += 1;
        }
        for k in m.keys() {
            out.push(k.to_string());
        }
    }
    out
}

pub fn union(a: &[Vec<char>], b: &[Vec<char>]) -> Vec<Vec<char>> {
    let mut m = HashMap::new();
    for item in a {
        m.insert(item.clone(), true);
    }
    for item in b {
        if !m.contains_key(item) {
            m.insert(item.clone(), true);
        }
    }

    let mut out = Vec::new();
    for (word, _) in &m {
        out.push(word.clone());
    }
    out
}

pub fn find(slice: &[Vec<char>], val: &[char]) -> isize {
    for (i, item) in slice.iter().enumerate() {
        if equal(item, val) {
            return i as isize;
        }
    }
    -1
}

pub fn sum(arr: &[isize]) -> isize {
    let mut res = 0;
    for v in arr {
        res += *v;
    }
    res
}

pub fn jaccard_similarity(str1: &str, str2: &str, split_length: usize) -> f32 {
    if str1.is_empty() || str2.is_empty() {
        return 0.0;
    }

    let splitted_str1: Vec<String> = if split_length == 0 {
        str1.split_whitespace().map(|s| s.to_string()).collect()
    } else {
        shingle_slice(str1, split_length)
    };
    let splitted_str2: Vec<String> = if split_length == 0 {
        str2.split_whitespace().map(|s| s.to_string()).collect()
    } else {
        shingle_slice(str2, split_length)
    };

    let rune_str1: Vec<Vec<char>> = splitted_str1.iter().map(|s| s.chars().collect()).collect();
    let rune_str2: Vec<Vec<char>> = splitted_str2.iter().map(|s| s.chars().collect()).collect();

    let union_str: Vec<Vec<char>> = union(&rune_str1, &rune_str2);
    let jacc = (rune_str1.len() + rune_str2.len() - union_str.len()) as f32;

    jacc / union_str.len() as f32
}

pub fn sorensen_dice_coefficient(str1: &str, str2: &str, split_length: usize) -> f32 {
    if str1.is_empty() && str2.is_empty() {
        return 0.0;
    }
    let shingle1 = shingle(str1, split_length);
    let shingle2 = shingle(str2, split_length);

    let mut intersection = 0.0;
    for (_, v1) in &shingle1 {
        if let Some(v2) = shingle2.get(&String::from(v1.to_string())) {
            intersection += (*v1 as f32).min(*v2 as f32);
        }
    }
    2.0 * intersection / (shingle1.values().sum::<usize>() as f32 + shingle2.values().sum::<usize>() as f32)
}

pub fn shingle(s: &str, k: usize) -> HashMap<String, usize> {
    let mut m = HashMap::new();
    if !s.is_empty() && k != 0 {
        let rune_s: Vec<char> = s.chars().collect();

        for i in 0..rune_s.len() - k + 1 {
            let key: String = rune_s[i..i + k].iter().collect();
            *m.entry(key).or_insert(0) += 1;
        }
    }
    m
}

pub fn qgram_similarity(str1: &str, str2: &str, split_length: usize) -> f32 {
    let splitted_str1 = shingle(str1, split_length);
    let splitted_str2 = shingle(str2, split_length);
    let res = qgram_distance_custom_ngram(&splitted_str1, &splitted_str2);
    1.0 - (res as f32 / (splitted_str1.values().sum::<usize>() as f32 + splitted_str2.values().sum::<usize>() as f32))
}

pub fn qgram_distance_custom_ngram(
    splitted_str1: &HashMap<String, usize>,
    splitted_str2: &HashMap<String, usize>,
) -> usize {
    let mut union = HashMap::new();
    for i in splitted_str1.keys().chain(splitted_str2.keys()) {
        union.insert(i.to_string(), 0);
    }

    let mut res = 0;
    for i in union.keys() {
        res += (*splitted_str1.get(i).unwrap_or(&0) as isize - *splitted_str2.get(i).unwrap_or(&0) as isize).abs() as usize;
    }

    res
}



