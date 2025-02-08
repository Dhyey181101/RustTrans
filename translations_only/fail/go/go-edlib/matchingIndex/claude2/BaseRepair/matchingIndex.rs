
fn matching_index(str1: &str, str2: &str, distance: isize) -> f32 {
    let rune_str1 = str1.chars().collect::<Vec<_>>();
    let rune_str2 = str2.chars().collect::<Vec<_>>();

    if rune_str1.len() >= rune_str2.len() {
        (rune_str1.len() - distance as usize) as f32 / rune_str1.len() as f32
    } else {
        (rune_str2.len() - distance as usize) as f32 / rune_str2.len() as f32
    }
}
