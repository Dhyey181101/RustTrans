
fn matchingIndex(str1: String, str2: String, distance: isize) -> f32 {
    let runeStr1: Vec<char> = str1.chars().collect();
    let runeStr2: Vec<char> = str2.chars().collect();

    if runeStr1.len() >= runeStr2.len() {
        let len_diff = (runeStr1.len() as isize) - distance;
        if len_diff < 0 {
            return 0.0;
        }
        return (len_diff as f32) / (runeStr1.len() as f32);
    }

    let len_diff = (runeStr2.len() as isize) - distance;
    if len_diff < 0 {
        return 0.0;
    }
    return (len_diff as f32) / (runeStr2.len() as f32);
}
