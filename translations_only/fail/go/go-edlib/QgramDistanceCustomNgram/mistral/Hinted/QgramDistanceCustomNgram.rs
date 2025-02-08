

use std::collections::HashMap;
use std::convert::TryInto;
use std::i8;

fn QgramDistanceCustomNgram(
    splitted_str1: HashMap<u8, isize>,
    splitted_str2: HashMap<u8, isize>,
) -> isize {
    let mut union = splitted_str1.clone();
    for (key, _) in &splitted_str2 {
        union.insert(*key, 0);
    }

    let mut res: isize = 0;
    for (key, value) in union {
        res += i8::try_from(splitted_str1.get(&key).unwrap_or(&0) - value)
            .unwrap()
            .abs() as isize;
    }

    res
}

#[test]
fn test_QgramDistanceCustomNgram() {
    assert_eq!(
        QgramDistanceCustomNgram(
            vec![('1', 7838315249666)].into_iter().collect(),
            vec![('1', 0)].into_iter().collect()
        ),
        7838315249666
    );
    assert_eq!(
        QgramDistanceCustomNgram(
            vec!['𐀀'.into()].into_iter().collect(),
            HashMap::new()
        ),
        0
    );
    assert_eq!(
        QgramDistanceCustomNgram(
            vec![(':', -360287970189059857)].into_iter().collect(),
            vec!['耀'.into() => 58264220667478016].into_iter().collect()
        ),
        418552190856537856
    );
    assert_eq!(
        QgramDistanceCustomNgram(
            vec![('a'.into(), 281474976711168)].into_iter().collect(),
            vec!['\0'.into() => 72057594021216256].into_iter().collect()
        ),
        72339068997927424
    );
}

