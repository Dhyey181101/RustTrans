
fn matching_index(str1: &str, str2: &str, distance: isize) -> f32 {
    let rune_str1: Vec<char> = str1.chars().collect();
    let rune_str2: Vec<char> = str2.chars().collect();
    if rune_str1.len() >= rune_str2.len() {
        return (rune_str1.len() as isize - distance) as f32 / rune_str1.len() as f32;
    }
    return (rune_str2.len() as isize - distance) as f32 / rune_str2.len() as f32;
}
