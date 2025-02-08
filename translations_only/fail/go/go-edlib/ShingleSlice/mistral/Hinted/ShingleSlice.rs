

use std::collections::HashMap;
use std::fmt;
use std::isize;

fn ShingleSlice(s: &str, k: isize) -> Vec<String> {
    let mut out = Vec::new();
    if s != "" && k != 0 {
        let rune_s: Vec<char> = s.chars().collect();
        let mut m = HashMap::new();
        for i in 0..(rune_s.len() as isize - k + 1) {
            let key = format!(
                "{}{}",
                &rune_s[i as usize..(i + k / 2) as usize]
                    .iter()
                    .map(|c| c.to_string())
                    .collect::<Vec<String>>()
                    .join(""),
                &rune_s[(i + k / 2) as usize..(i + k) as usize]
                    .iter()
                    .map(|c| c.to_string())
                    .collect::<Vec<String>>()
                    .join("")
            );
            let value = m.get(&key).unwrap_or(&0) + 1;
            m.insert(key.clone(), value);
        }
        for (k, v) in m {
            out.push(k);
        }
    }
    out
}

struct DisplayVec(Vec<String>);

impl fmt::Display for DisplayVec {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0.join(""))
    }
}

fn main() {
    let test_str_1 = "@";
    let test_str_2 = "";
    let test_int_1 = -71941692576945047;
    let test_int_2 = -71987221015552000;
    let test_int_3 = 280375465084928;
    let test_int_4 = 71777214277943040;
    println!("{:?}", ShingleSlice(test_str_1, test_int_1));
    println!("{:?}", ShingleSlice(test_str_1, test_int_2));
    println!("{:?}", ShingleSlice(test_str_1, test_int_3));
    println!("{:?}", ShingleSlice(test_str_2, test_int_4));

    let vec_c = vec!['a', 'b', 'c'];
    let dbg_vec_c = DisplayVec(
        vec_c.into_iter()
            .map(|c| c.to_string())
            .collect(),
    );
    println!("{}", dbg_vec_c);
}

