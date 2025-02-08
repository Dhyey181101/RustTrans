
fn matchingIndex(str1: String, str2: String, distance: isize) -> f32 {
    let runeStr1: Vec<char> = str1.chars().collect();
    let runeStr2: Vec<char> = str2.chars().collect();

    if runeStr1.len() >= runeStr2.len() {
        return (runeStr1.len() - distance as usize) as f32 / runeStr1.len() as f32;
    }
    return (runeStr2.len() - distance as usize) as f32 / runeStr2.len() as f32;
}
