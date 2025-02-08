
use std::boxed::Box;

pub fn sum(arr: Vec<isize>) -> isize {
    let mut res: isize = 0;
    for v in arr {
        res += v;
    }
    res
}
