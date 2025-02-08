
use std::collections::HashMap;

fn QgramDistanceCustomNgram(splittedStr1: HashMap<String, isize>, splittedStr2: HashMap<String, isize>) -> isize {
    let mut union = HashMap::new();
    for i in splittedStr1.keys() {
        union.insert(i.clone(), 0);
    }
    for i in splittedStr2.keys() {
        union.insert(i.clone(), 0);
    }

    let mut res = 0;
    for i in union.keys() {
        res += (splittedStr1.get(i).unwrap_or(&0) - splittedStr2.get(i).unwrap_or(&0)).abs() as isize;
    }

    res
}
