
use std::cmp::max;

pub fn lcse_ditance(str1: &str, str2: &str) -> isize {
    if str1.len() == 0 {
        return str2.len() as isize;
    } else if str2.len() == 0 {
        return str1.len() as isize;
    } else if str1 == str2 {
        return 0;
    }

    let lcs = lcs(str1, str2);
    return (str1.len() as isize) + (str2.len() as isize) - 2 * lcs as isize;
}

pub fn lcs(str1: &str, str2: &str) -> usize {
    let rune_str1: Vec<char> = str1.chars().collect();
    let rune_str2: Vec<char> = str2.chars().collect();

    let len1 = rune_str1.len();
    let len2 = rune_str2.len();

    let mut dp = vec![vec![0; len2 + 1]; len1 + 1];

    for i in 1..=len1 {
        for j in 1..=len2 {
            if rune_str1[i - 1] == rune_str2[j - 1] {
                dp[i][j] = dp[i - 1][j - 1] + 1;
            } else {
                dp[i][j] = max(dp[i - 1][j], dp[i][j - 1]);
            }
        }
    }

    dp[len1][len2]
}
