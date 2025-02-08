

use std::collections::HashMap;
use std::boxed::Box;

fn QgramDistanceCustomNgram(
    splitted_str1: &HashMap<String, isize>,
    splitted_str2: &HashMap<String, isize>,
) -> isize {
    let mut union = HashMap::new();
    for (k, _) in splitted_str1.iter() {
        union.insert(k.clone(), 0);
    }
    for (k, _) in splitted_str2.iter() {
        if !union.contains_key(k) {
            union.insert(k.clone(), 0);
        }
    }

    let mut res: isize = 0;
    for (k, v) in union.iter() {
        let val1 = *splitted_str1.get(k).unwrap();
        let val2 = *splitted_str2.get(k).unwrap();
        res += (val1 - val2).abs() as isize;
    }

    res
}

