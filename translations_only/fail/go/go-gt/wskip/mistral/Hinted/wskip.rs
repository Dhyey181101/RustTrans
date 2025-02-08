

use std::str;

fn wskip(s: &str) -> &str {
s.find(|c: char| c != ' ' && c != '\t').map_or("", |i| &s[i..])
}

#[test]
fn test_wskip() {
let test_cases = vec![
("o ", "o "),
(r"\n\n", r"\n\n"),
(r"\n n ", r"n "),
(r"%#n ", r"%#n "),
];

for (input, expected) in test_cases {
let result = wskip(input);
assert_eq!(result, expected);
}
}

