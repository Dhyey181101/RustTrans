
fn jaro_similarity(str1: &str, str2: &str) -> f32 {
    let mut runestr1: Vec<char> = str1.chars().collect();
    let mut runestr2: Vec<char> = str2.chars().collect();

    let runestr1len = runestr1.len();
    let runestr2len = runestr2.len();
    if runestr1len == 0 || runestr2len == 0 {
        return 0.0;
    } else if runestr1 == runestr2 {
        return 1.0;
    }

    let mut r#match = 0;
    let max_dist = isize_max(runestr1len as isize, runestr2len as isize) / 2 - 1;
    let mut str1table: Vec<isize> = vec![0; runestr1len];
    let mut str2table: Vec<isize> = vec![0; runestr2len];

    for i in 0..runestr1len {
        for j in isize_max(0, i as isize - max_dist)..isize_min(runestr2len as isize, i as isize + max_dist + 1) {
            if runestr1[i as usize] == runestr2[j as usize] && str2table[j as usize] == 0 {
                str1table[i as usize] = 1;
                str2table[j as usize] = 1;
                r#match += 1;
                break;
            }
        }
    }
    if r#match == 0 {
        return 0.0;
    }

    let mut t = 0.0;
    let mut p = 0;
    for i in 0..runestr1len {
        if str1table[i as usize] == 1 {
            while str2table[p] == 0 {
                p += 1;
            }
            if runestr1[i as usize] != runestr2[p] {
                t += 1.0;
            }
            p += 1;
        }
    }
    t /= 2.0;

    (r#match as f32 / runestr1len as f32
        + r#match as f32 / runestr2len as f32
        + (r#match as f32 - t) / r#match as f32)
        / 3.0
}

fn isize_max(a: isize, b: isize) -> isize {
    if b > a {
        b
    } else {
        a
    }
}

fn isize_min(a: isize, b: isize) -> isize {
    if b < a {
        b
    } else {
        a
    }
}
