
fn equal(a: &[char], b: &[char]) -> bool {
if a.len() != b.len() {
return false;
}
for i in 0..a.len() {
if a[i] != b[i] {
return false;
}
}
return true;
}
