
use std::collections::HashMap;

fn QgramDistanceCustomNgram(splittedStr1: HashMap<String, isize>, splittedStr2: HashMap<String, isize>) -> isize {
    let mut union = HashMap::new();

    for key in splittedStr1.keys() {
        union.insert(key.clone(), 0);
    }

    for key in splittedStr2.keys() {
        union.entry(key.clone()).or_insert(0);
    }

    let mut res = 0;
    for key in union.keys() {
        let val1 = *splittedStr1.get(key).unwrap_or(&0);
        let val2 = *splittedStr2.get(key).unwrap_or(&0);
        res += (val1 - val2).abs() as isize;
    }

    res
}
